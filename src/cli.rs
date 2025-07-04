use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run {
        #[arg(short = 'd')]
        display: bool,

        #[arg(last = true)]
        commands: Vec<String>,
    },
    Watch {
        #[arg(short = 'd')]
        display: bool,

        process_id: u32,
    },
}
