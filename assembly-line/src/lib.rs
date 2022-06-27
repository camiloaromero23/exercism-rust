// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let cars_per_hour = 221.0;
    let rate: f64;

    if speed > 0 && speed < 5 {
        rate = 1.0
    } else if speed < 9 {
        rate = 0.9
    } else {
        rate = 0.77
    }
    cars_per_hour * speed as f64 * rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
