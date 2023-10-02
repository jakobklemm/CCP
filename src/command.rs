//! # Command

pub struct Command {
    source: String,
    target: String,
    from: Option<usize>,
    to: Option<usize>,
}

impl Comamnd {
    fn ingest(source: String, target: String) -> Self {
        Self {
            source,
            target,
            from: None,
            to: None
        }
    }
}
