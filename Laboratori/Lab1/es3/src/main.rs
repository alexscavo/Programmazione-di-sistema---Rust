pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        Clock{hours, minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {


        let mut hours: i32 = self.hours;
        let mut mins: i32 = self.minutes;

        hours = hours + minutes /60;

        minutes = minutes - minutes / 60;

        if mins + minutes >= 60 {
            if hours + 1 >= 24 {
                hours = hours - 23;
            }
            mins = mins - 60;
        }

        mins = mins + minutes;

        Clock::new(hours, mins)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}


fn main() {


    let clock = Clock::new(8, 0);

    println!("{}", clock.to_string());

    let clock1 = Clock::new(15, 55);
    let clock2 = Clock::new(16, 0);
    let clock3 = Clock::new(15, 55);

    println!("clock1 == clock2? {}", clock1 == clock3);

    let clock = Clock::new(5, 32).add_minutes(1500);
    println!("{}", clock.to_string());
}
