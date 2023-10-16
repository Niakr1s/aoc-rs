use super::*;

impl From<&str> for Gate {
    fn from(s: &str) -> Self {
        Gate(s.to_owned())
    }
}
