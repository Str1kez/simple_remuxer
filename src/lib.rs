mod cli;
mod models;

pub use models::Config;
use models::CustomError;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    error::Error,
    io,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
};

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

fn make_remux(config: &Config, dirs: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let workers: RwLock<Vec<Child>> = RwLock::new(Vec::with_capacity(dirs.len()));
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);
    let input_path = config.input_path();
    let output_path = config.output_path();

    ctrlc::set_handler(move || running_clone.store(false, Ordering::SeqCst))?;

    dirs.into_par_iter().for_each(|dir| {
        let mut workers = workers.write().unwrap();
        let child_process = Command::new("sh")
            .args([
                "-c",
                &format!(
                    "ffmpeg -i bluray:{input_path}/{dir} -map 0 -c copy {output_path}/{dir}.mkv"
                ),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap_or_else(|_| panic!("Couldn't read dir: {dir}"));
        workers.push(child_process);
    });

    let mut is_error = false;
    while running.load(Ordering::SeqCst) {
        if workers.write().unwrap().iter_mut().all(|worker| -> bool {
            match worker.try_wait() {
                Ok(Some(status)) => {
                    // TODO: сообщение об успехе или ошибку
                    is_error |= !status.success();
                    true
                }
                // TODO: добавляем тут крутилку
                Ok(None) => false,
                Err(e) => {
                    eprintln!("{e}");
                    true
                }
            }
        }) {
            return if is_error {
                Err(Box::new(CustomError::new(
                    "Something wrong in ffmpeg subprocesses",
                )))
            } else {
                Ok(())
            };
        }
    }

    workers.write().unwrap().iter_mut().for_each(|worker| {
        let _ = worker.kill();
    });

    Ok(())
}
