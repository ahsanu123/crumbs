#[derive(Default, Clone)]
pub enum Units {
    #[default]
    Celcius,
    Reamur,
    Fahrenheit,
}

impl From<Units> for tmd_ui::Units {
    fn from(value: Units) -> Self {
        match value {
            Units::Celcius => tmd_ui::Units::Celcius,
            Units::Reamur => tmd_ui::Units::Reamur,
            Units::Fahrenheit => tmd_ui::Units::Fahrenheit,
        }
    }
}
