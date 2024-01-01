mod cli;
mod core;
pub mod models;

use core::make;
use models::Config;
use std::{error::Error, io, process::Command};
#[allow(clippy::missing_errors_doc)]
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let path = config.input_path();
    let dirs_command = Command::new("ls").arg(path).output()?;
    if let Some(bd_dirs) = String::from_utf8_lossy(&dirs_command.stdout).strip_suffix('\n') {
        make(config, bd_dirs.split('\n').collect::<Vec<&str>>())?;
    } else {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "Couldn't parse dirs",
        )));
    }
    Ok(())
}
