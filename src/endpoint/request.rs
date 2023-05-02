use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct Request {
    from: String,
    max_duration: i32,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.from, self.max_duration)
    }
}
