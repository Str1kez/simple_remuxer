use crate::cli::parse_args;
use std::error::Error;

pub struct Config {
    input_path: String,
    output_path: String,
}

impl Config {
    #[allow(clippy::missing_errors_doc)]
    pub fn new<T: Iterator<Item = String>>(args: T) -> Result<Config, Box<dyn Error>> {
        let (mut input_path, mut output_path) = parse_args(args)?;
        if let Some(deleted_slash) = input_path.strip_suffix('/') {
            input_path = deleted_slash.to_owned();
        }
        if let Some(deleted_slash) = output_path.strip_suffix('/') {
            output_path = deleted_slash.to_owned();
        }
        Ok(Config {
            input_path,
            output_path,
        })
    }

    #[must_use]
    pub fn input_path(&self) -> &str {
        &self.input_path
    }
    #[must_use]
    pub fn output_path(&self) -> &str {
        &self.output_path
    }
}
