#[derive(Default, Clone)]
pub enum Tabs {
    #[default]
    Home,
    Bluetooth,
    Unit,
}

impl From<Tabs> for tmd_ui::Tabs {
    fn from(value: Tabs) -> Self {
        match value {
            Tabs::Home => tmd_ui::Tabs::Home,
            Tabs::Bluetooth => tmd_ui::Tabs::Ble,
            Tabs::Unit => tmd_ui::Tabs::Unit,
        }
    }
}
