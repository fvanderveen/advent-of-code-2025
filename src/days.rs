// « add day import »

pub struct Day {
    pub puzzle1: fn(input: &String) -> Result<String, String>,
    pub puzzle2: fn(input: &String) -> Result<String, String>
}

pub fn get_day(day: i32) -> Result<Day, String> {
    match day {
        // « add day match »
        _ => Err(format!("No implementation yet for day {}", day))
    }
}