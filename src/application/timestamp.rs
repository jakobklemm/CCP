//! # Timestamp

use serde::{Deserialize, Serialize};
use std::ops::{Div, Sub};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Timestamp {
    hours: i32,
    minutes: i32,
    seconds: i32,
    millis: i32,
}

impl Sub for Timestamp {
    type Output = Timestamp;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            hours: self.hours - other.hours,
            minutes: self.minutes - other.minutes,
            seconds: self.seconds - other.seconds,
            millis: self.millis - other.millis,
        }
    }
}

impl Div for Timestamp {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.to_seconds() as f32 / rhs.to_seconds() as f32
    }
}

impl ToString for Timestamp {
    fn to_string(&self) -> String {
        format!(
            "{}:{}:{}.{}",
            self.hours, self.minutes, self.seconds, self.millis
        )
    }
}

impl Timestamp {
    fn to_seconds(&self) -> i32 {
        3600 * self.hours + 60 * self.minutes + self.seconds
    }

    pub fn from_input(lines: &[String]) -> String {
        for l in lines {
            return l.clone();
        }
        String::new()
    }

    pub fn from_str(line: impl ToString) -> Option<Self> {
        let binding = line.to_string();
        let parts = binding.split(":");
        let mut ts = Timestamp::default();
        for (i, part) in parts.into_iter().enumerate() {
            match i {
                0 => {
                    // println!("{:?}", part);
                    // ts.hours = part.parse().unwrap();
                    if let Ok(h) = part.parse() {
                        ts.hours = h;
                    } else {
                        return None;
                    }
                }
                1 => {
                    // ts.minutes = part.parse().unwrap();
                    if let Ok(m) = part.parse() {
                        ts.minutes = m;
                    } else {
                        return None;
                    }
                }
                2 => {
                    let inner_parts = part.split(".");
                    for (j, p) in inner_parts.into_iter().enumerate() {
                        match j {
                            0 => {
                                // ts.seconds = p.parse().unwrap();
                                if let Ok(s) = p.parse() {
                                    ts.seconds = s;
                                } else {
                                    return None;
                                }
                            }
                            1 => {
                                // ts.millis = p.parse().unwrap();
                                if let Ok(m) = p.parse() {
                                    ts.millis = m;
                                } else {
                                    return None;
                                }
                            }
                            _ => {
                                return None;
                            }
                        }
                    }
                }
                _ => {
                    return None;
                }
            }
        }
        // println!("{:?}", ts);
        Some(ts)
    }
}
