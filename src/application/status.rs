//! # Status

use super::id::Id;

#[derive(Debug, Clone)]
pub enum Status {
    /// INV: 0 <= i <= 100
    First(u16),
    Second(u16),
    Third(u16),
    Complete(Id),
}

impl Status {
    pub fn get_first(&self) -> u16 {
        match self {
            Self::First(i) => *i,
            _ => 0,
        }
    }

    pub fn get_second(&self) -> u16 {
        match self {
            Self::Second(i) => *i,
            _ => 0,
        }
    }

    pub fn get_third(&self) -> u16 {
        match self {
            Self::Third(i) => *i,
            _ => 0,
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::First(0)
    }
}
