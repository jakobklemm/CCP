//! # Job

use super::timestamp::Timestamp;

pub struct Job {
    start: Timestamp,
    end: Timestamp,
    title: String,
    description: String,
    tags: Vec<String>,
}
