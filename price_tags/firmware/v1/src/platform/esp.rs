extern crate alloc;

use alloc::boxed::Box;
use core::{
    fmt::{Debug, Display},
    future::Future,
    net::Ipv4Addr,
    pin::Pin,
    str::FromStr,
};
use edge_http::io::server::Handler;
use edge_http::io::{
    Error,
    server::{Connection, Server},
};
use edge_nal::{TcpBind, io::Write as _};
use edge_nal_embassy::{Tcp, TcpBuffers};
use embassy_executor::Spawner;
use embassy_net::{Ipv4Cidr, Runner, Stack, StackResources, StaticConfigV4};
use embassy_time::{Duration, Timer};
use esp_hal::peripherals::WIFI;
use esp_hal::rng::Rng;
use esp_radio::wifi::AccessPointStationEventInfo;
use esp_radio::wifi::{Config, ControllerConfig, Interface, WifiController, ap::AccessPointConfig};

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write($val);
        x
    }};
}

pub fn spawn_http_server(
    spawner: &Spawner,
    wifi: WIFI<'static>,
    ssid_name: &str,
    http_handle: impl Handler + 'static,
    option_ip_addr: Option<&'static str>,
) {
    let ip_addr = option_ip_addr.unwrap_or("192.168.2.1");

    let accespoint_conf = AccessPointConfig::default().with_ssid(ssid_name);
    let access_point_config = Config::AccessPoint(accespoint_conf);

    defmt::info!("Starting wifi");

    let (controller, interface) = esp_radio::wifi::new(
        wifi,
        ControllerConfig::default().with_initial_config(access_point_config),
    )
    .expect("fail to create new wifi");

    defmt::info!("Wifi started!");

    let gw_ip_addr = Ipv4Addr::from_str(ip_addr).expect("failed to parse gateway ip");

    let config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: Ipv4Cidr::new(gw_ip_addr, 24),
        gateway: Some(gw_ip_addr),
        dns_servers: Default::default(),
    });

    let rng = Rng::new();
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    // Init network stack
    let (stack, runner) = embassy_net::new(
        interface.access_point,
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    let http_server_task_factory = HttpServerTaskFactory::new(stack, http_handle);

    spawner.spawn(connection(controller).unwrap());
    spawner.spawn(net_task(runner).unwrap());
    spawner.spawn(run_dhcp(stack, ip_addr).unwrap());
    spawner.spawn(http_server_task(http_server_task_factory).unwrap());
}

#[embassy_executor::task]
async fn run_dhcp(stack: Stack<'static>, gw_ip_addr: &'static str) {
    use core::net::{Ipv4Addr, SocketAddrV4};

    use edge_dhcp::{
        io::{self, DEFAULT_SERVER_PORT},
        server::{Server, ServerOptions},
    };
    use edge_nal::UdpBind;
    use edge_nal_embassy::{Udp, UdpBuffers};

    let ip = Ipv4Addr::from_str(gw_ip_addr).expect("dhcp task failed to parse gw ip");

    let mut buf = [0u8; 1500];

    let mut gw_buf = [Ipv4Addr::UNSPECIFIED];

    let buffers = UdpBuffers::<3, 1024, 1024, 10>::new();
    let unbound_socket = Udp::new(stack, &buffers);
    let mut bound_socket = unbound_socket
        .bind(core::net::SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::UNSPECIFIED,
            DEFAULT_SERVER_PORT,
        )))
        .await
        .unwrap();

    loop {
        _ = io::server::run(
            &mut Server::<_, 64>::new_with_et(ip),
            &ServerOptions::new(ip, Some(&mut gw_buf)),
            &mut bound_socket,
            &mut buf,
        )
        .await
        .inspect_err(|_e| defmt::warn!("DHCP server error: [redacted]",));
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn connection(controller: WifiController<'static>) {
    defmt::info!("start connection task");
    loop {
        let ev = controller
            .wait_for_access_point_connected_event_async()
            .await
            .unwrap();
        match ev {
            AccessPointStationEventInfo::Connected(info) => {
                defmt::info!("Station connected: {:?}", info)
            }
            AccessPointStationEventInfo::Disconnected(info) => {
                defmt::info!("Station disconnected: {:?}", info)
            }
        }
        Timer::after(Duration::from_millis(5000)).await
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface<'static>>) {
    runner.run().await
}

#[embassy_executor::task]
async fn http_server_task(mut http_server_factory: HttpServerTaskFactory) {
    http_server_factory.run().await;
}

pub struct HttpServerTaskFactory {
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
}

impl HttpServerTaskFactory {
    pub fn new(stack: Stack<'static>, http_handler: impl Handler + 'static) -> Self {
        Self {
            // Erase the future because Handler's generic methods are not dyn-compatible.
            future: Box::pin(async move {
                defmt::info!("HTTP server: http://192.168.2.1:8881/");

                stack.wait_link_up().await;
                stack.wait_config_up().await;

                let buffers = TcpBuffers::<1, 1024, 1024>::new();
                let tcp = Tcp::new(stack, &buffers);
                let acceptor = tcp
                    .bind(core::net::SocketAddr::from(([0, 0, 0, 0], 8881)))
                    .await
                    .expect("Failed to bind HTTP listener");
                let mut server = Server::<1, 1024, 16>::new();
                loop {
                    if let Err(e) = server.run(None, &acceptor, &http_handler).await {
                        defmt::info!("HTTP server error: {:?}", defmt::Debug2Format(&e));
                        Timer::after(Duration::from_secs(1)).await;
                    }
                }
            }),
        }
    }

    pub async fn run(&mut self) {
        self.future.as_mut().await;
    }
}

pub struct HelloWorldHttpHandler;

impl Handler for HelloWorldHttpHandler {
    type Error<E>
        = Error<E>
    where
        E: Debug;

    async fn handle<T, const N: usize>(
        &self,
        _task_id: impl Display + Copy,
        conn: &mut Connection<'_, T, N>,
    ) -> Result<(), Self::Error<T::Error>>
    where
        T: edge_nal::io::Read + edge_nal::io::Write + edge_nal::TcpSplit,
    {
        conn.initiate_response(200, Some("OK"), &[("Content-Type", "text/plain")])
            .await?;
        conn.write_all(b"hello world!!!").await?;

        Ok(())
    }
}
