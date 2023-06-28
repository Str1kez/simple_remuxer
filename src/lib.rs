mod cli;
mod core;
mod models;

use crate::core::make_remux;
pub use models::Config;
use std::{error::Error, io, process::Command};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let path = config.input_path();
    let dirs_command = Command::new("ls").arg(path).output()?;
    if let Some(bd_dirs) = String::from_utf8_lossy(&dirs_command.stdout).strip_suffix('\n') {
        make_remux(config, bd_dirs.split('\n').collect::<Vec<&str>>())?;
    } else {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "Couldn't parse dirs",
        )));
    }
    Ok(())
}
