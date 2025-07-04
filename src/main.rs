mod api;
mod cli;

use crate::api::{disable_sleep, enable_sleep, find_process};
use crate::cli::{Cli, Commands};
use clap::Parser;
use std::error::Error;
use std::process::{Command, ExitCode};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match match cli.command {
        Commands::Run { display, commands } => main_run(display, commands),
        Commands::Watch {
            display,
            process_id,
        } => main_watch(display, process_id),
    } {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{:?}", e);
            ExitCode::FAILURE
        }
    }
}

fn main_run(display: bool, commands: Vec<String>) -> Result<ExitCode, Box<dyn Error>> {
    if commands.is_empty() {
        eprintln!("Missing arguments.");
        return Ok(ExitCode::FAILURE);
    }

    let mut cmd = Command::new(&commands[0]);
    let mut child = if commands.len() == 1 {
        cmd.spawn()?
    } else {
        cmd.args(commands.iter().skip(1)).spawn()?
    };

    let exit_status = {
        ctrlc::set_handler(move || {})?;
        let _waker = Waker::new(display);
        child.wait()?
    };

    if exit_status.success() {
        Ok(ExitCode::SUCCESS)
    } else if let Some(code) = exit_status.code() {
        // https://github.com/rust-lang/rust/issues/111688
        Ok(ExitCode::from(u8::try_from(code)?))
    } else {
        Ok(ExitCode::FAILURE)
    }
}

fn main_watch(display: bool, process_id: u32) -> Result<ExitCode, Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    let _waker = Waker::new(display);
    while running.load(Ordering::SeqCst) && find_process(process_id)? {
        sleep(Duration::from_millis(100));
    }
    Ok(ExitCode::SUCCESS)
}

struct Waker {}

impl Waker {
    fn new(display: bool) -> Waker {
        disable_sleep(display);
        Waker {}
    }
}

impl Drop for Waker {
    fn drop(&mut self) {
        enable_sleep();
    }
}
