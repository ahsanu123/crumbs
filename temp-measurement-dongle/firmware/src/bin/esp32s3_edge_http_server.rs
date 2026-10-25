#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use alloc::format;
use core::{
    fmt::{Debug, Display},
    net::Ipv4Addr,
    str::FromStr,
};
use edge_http::io::{
    Body, Error,
    server::{Connection, Server},
};
use edge_http::{Method, io::server::Handler};
use edge_nal::TcpBind;
use edge_nal_embassy::{Tcp, TcpBuffers};
use embassy_executor::Spawner;
use embassy_net::{Ipv4Cidr, Runner, Stack, StackResources, StaticConfigV4};
use embassy_time::{Duration, Timer};
use embedded_io_async::{Read, Write};
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{clock::CpuClock, ram, rng::Rng, timer::timg::TimerGroup};
use esp_println::println;
use esp_radio::wifi::AccessPointStationEventInfo;
use esp_radio::wifi::{Config, ControllerConfig, Interface, WifiController, ap::AccessPointConfig};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// Full `.dram2_uninit` region for this chip.
const DRAM2_UNINIT_SIZE: usize = {
    let range: core::ops::Range<usize> = esp_metadata_generated::memory_range!("DRAM2_UNINIT");
    range.end - range.start
};

// When you are okay with using a nightly compiler it's better to use https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write($val);
        x
    }};
}

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) {
    esp_alloc::heap_allocator!(#[ram(reclaimed)] size: DRAM2_UNINIT_SIZE);
    esp_alloc::heap_allocator!(size: 46 * 1024);

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());

    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    let accespoint_conf = AccessPointConfig::default().with_ssid("esphttp");
    let access_point_config = Config::AccessPoint(accespoint_conf);

    println!("Starting wifi");

    let (controller, interface) = esp_radio::wifi::new(
        peripherals.WIFI,
        ControllerConfig::default().with_initial_config(access_point_config),
    )
    .expect("fail to create new wifi");

    println!("Wifi started!");

    let gw_ip_addr_str = "192.168.2.1";
    let gw_ip_addr = Ipv4Addr::from_str(gw_ip_addr_str).expect("failed to parse gateway ip");

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

    spawner.spawn(connection(controller)).unwrap();
    spawner.spawn(net_task(runner)).unwrap();
    spawner.spawn(run_dhcp(stack, gw_ip_addr_str)).unwrap();
    spawner.spawn(http_server_task(stack)).unwrap();

    loop {
        defmt::info!("tick....");

        Timer::after(Duration::from_secs(3)).await;
    }
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
    println!("start connection task");
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
async fn http_server_task(stack: Stack<'static>) {
    stack.wait_link_up().await;
    stack.wait_config_up().await;

    let buffers = TcpBuffers::<1, 1024, 1024>::new();
    let tcp = Tcp::new(stack, &buffers);
    let acceptor = tcp
        .bind(core::net::SocketAddr::from(([0, 0, 0, 0], 8881)))
        .await
        .expect("Failed to bind HTTP listener");
    let mut server = Server::<1, 1024, 16>::new();

    println!("HTTP server: http://192.168.2.1:8881/");

    loop {
        if let Err(e) = server.run(None, &acceptor, HttpHandler).await {
            println!("HTTP server error: {:?}", e);
            Timer::after(Duration::from_secs(1)).await;
        }
    }
}

struct HttpHandler;

impl Handler for HttpHandler {
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
        T: Read + Write,
    {
        let (headers, body) = conn.split();
        let path = headers.path;

        defmt::info!("check for request");
        if headers.method == Method::Post && path.starts_with("/upload") {
            let mut string = alloc::string::String::new();

            if let Some(filename) = path.split('/').nth(2) {
                string.push_str(filename);
            };

            let mut buffer: [u8; 1024] = [0; 1024];
            loop {
                let bufsize = body.read(&mut buffer).await?;
                if bufsize == 0 {
                    break;
                }

                string
                    .push_str(core::str::from_utf8(&buffer[..bufsize]).unwrap_or("<invalid utf8>"));
            }

            conn.initiate_response(200, Some("OK"), &[("Content-Type", "text/plain")])
                .await?;

            let msg =
                format!("Hello world!, this string you send to server {string:?}, path: {path}",);
            defmt::info!("write all request.");
            conn.write_all(msg.as_bytes()).await?;
        } else {
            conn.initiate_response(500, Some("not found"), &[]).await?;
        }

        Ok(())
    }
}
