use anyhow::Context;
use clap::Parser;
use cs25_303_core::database::DatabaseConfig;
use sqlx::PgPool;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};
pub mod manual_test;
use std::path::{Path, PathBuf};
#[derive(Debug, Clone, Parser)]
pub struct CLI {
    #[clap(flatten)]
    pub database: DatabaseConfig,
    #[clap(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Commands {
    RunManualTests { path: PathBuf },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();
    load_logging()?;
    let database = cs25_303_core::database::connect(cli.database.try_into()?, true).await?;
    match cli.command {
        Commands::RunManualTests { path } => {
            println!("Running manual tests from path: {:?}", path);
            run_manual_dir(path, database).await?;
        }
    }
    Ok(())
}
fn load_logging() -> anyhow::Result<()> {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    tracing_subscriber::registry()
        .with(
            stdout_log.with_filter(
                filter::Targets::new()
                    .with_target("cs25_303_test_data", LevelFilter::TRACE)
                    .with_target("cs25_303_core", LevelFilter::TRACE)
                    .with_target("sqlx", LevelFilter::INFO),
            ),
        )
        .init();
    Ok(())
}
pub async fn run_manual_dir(path: PathBuf, database: PgPool) -> anyhow::Result<()> {
    let folders = std::fs::read_dir(path)?;
    for folder in folders {
        let folder = folder?;
        let path = folder.path();
        if !path.is_dir() {
            continue;
        }
        if does_file_name_start_with(&path, "participant")? {
            let mut tx = database.begin().await?;

            manual_test::create_participant_from_dir(path, &mut tx).await?;
            tx.commit().await?;
        }
    }
    Ok(())
}

pub fn does_file_name_start_with(path: impl AsRef<Path>, start: &str) -> anyhow::Result<bool> {
    let file_name = path
        .as_ref()
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("No file name"))?
        .to_str()
        .context("Invalid file name")?;
    Ok(file_name.starts_with(start))
}
