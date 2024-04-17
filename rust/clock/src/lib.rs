use std::{fmt::Display, ops::Add};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: (hours + minutes.div_euclid(60)).rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        self + Clock::new(0, minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Add for Clock {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Clock::new(self.hours + rhs.hours, self.minutes + rhs.minutes)
    }
}
