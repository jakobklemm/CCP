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
    pub fn get_perc(&self) -> (u16, u16, u16) {
        match self {
            Self::First(i) => (*i, 0, 0),
            Self::Second(i) => (100, *i, 0),
            Self::Third(i) => (100, 100, *i),
            _ => (0, 0, 0),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::First(0)
    }
}
