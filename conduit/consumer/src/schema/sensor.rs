#[derive(Debug)]
pub struct Sensor {
    pub id: i32,
    pub plant_id: Option<i32>,
    pub dry_adc: i32,
    pub wet_adc: i32,
}
