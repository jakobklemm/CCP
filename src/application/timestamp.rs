//! # Timestamp

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::ops::{Div, Sub};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
    pub fn to_seconds(&self) -> i32 {
        3600 * self.hours + 60 * self.minutes + self.seconds
    }

    pub fn from_input(lines: &[String]) -> Result<Self> {
        for l in lines {
            return Self::from_str(l);
        }
        Err(anyhow!("invalid timestamp!"))
    }

    pub fn from_str(line: impl ToString) -> Result<Self> {
        let binding = line.to_string();
        let parts = binding.split(":");
        let mut ts = Timestamp::default();
        for (i, part) in parts.into_iter().enumerate() {
            match i {
                0 => {
                    // println!("{:?}", part);
                    // ts.hours = part.parse().unwrap();
                    ts.hours = part.parse()?;
                }
                1 => {
                    ts.minutes = part.parse()?;
                }
                2 => {
                    let inner_parts = part.split(".");
                    for (j, p) in inner_parts.into_iter().enumerate() {
                        match j {
                            0 => {
                                ts.seconds = p.parse()?;
                            }
                            1 => {
                                ts.millis = p.parse()?;
                            }
                            _ => {
                                return Err(anyhow!("invalid timestamp!"));
                            }
                        }
                    }
                }
                _ => {
                    return Err(anyhow!("invalid timestamp!"));
                }
            }
        }
        // println!("{:?}", ts);
        Ok(ts)
    }
}
