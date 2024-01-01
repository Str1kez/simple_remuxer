use std::{error::Error, fmt};

#[derive(Debug)]
pub struct Custom {
    message: String,
}

impl Custom {
    #[must_use]
    pub fn new(message: &str) -> Self {
        Custom {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Custom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for Custom {}
