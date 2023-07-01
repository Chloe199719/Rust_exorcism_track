
#[derive(Debug ,PartialEq)]
pub struct Clock{
    hours:i32,
    minutes: i32
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        // let mut calc_hours = hours;
        // let mut calc_minutes = minutes;
        // calc_hours += calc_minutes / 60;
        // calc_minutes = calc_minutes % 60;
        // Clock {
        //     hours:  calc_hours % 24,
        //     minutes: calc_minutes,
        // }
        let total_minutes = hours * 60 + minutes;
        let normalized_minutes = ((total_minutes % 1440) + 1440) % 1440;
        Clock {
            hours: normalized_minutes / 60,
            minutes: normalized_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
