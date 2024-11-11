pub mod app;
pub mod config;
pub mod logging;
pub mod utils;
use std::{
    path::PathBuf,
    sync::atomic::{self, AtomicUsize},
};

use clap::{Parser, Subcommand, ValueEnum};
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ExportOptions {
    OpenAPI,
}
#[derive(Parser)]
struct Command {
    #[clap(subcommand)]
    sub_command: SubCommands,
}
#[derive(Subcommand, Clone, Debug)]
enum SubCommands {
    /// Starts the application
    Start {
        /// The thd-helper config file
        #[clap(short, long, default_value = "cs25-303.toml")]
        config: PathBuf,
    },
    /// Export the API documentation
    Export {
        export: ExportOptions,
        location: PathBuf,
    },
    Info,
}
fn info() {
    println!("Welcome to the capstone project for CS-25-303 at VCU.");
    println!("This project source is available at https://github.com/VCU-CS-Capstone/CS-25-303-SON-clinicians");
}
fn main() -> anyhow::Result<()> {
    let command = Command::parse();

    match command.sub_command {
        SubCommands::Start { config } => return start_app(config),
        SubCommands::Export { .. } => {
            eprintln!("Not implemented yet");
        }
        SubCommands::Info => {
            info();
        }
    }
    Ok(())
}

fn start_app(config: PathBuf) -> anyhow::Result<()> {
    let config = config::load_config(Some(config))?;
    // Ensure we have a default crypto provider. This caused a bug in the past. I don't know if we will need it.
    if rustls::crypto::ring::default_provider()
        .install_default()
        .is_err()
    {
        eprintln!("Default Crypto Provider already installed. This is not an error. But it should be reported.");
    }
    // Initlaize the tokio runtime

    let tokio = tokio::runtime::Builder::new_current_thread()
        .thread_name_fn(thread_name)
        .enable_all()
        .build()?;
    tokio.block_on(app::start_web_server(config))
}
fn thread_name() -> String {
    static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
    let id = ATOMIC_ID.fetch_add(1, atomic::Ordering::SeqCst);
    format!("cs-25-303-{}", id)
}
