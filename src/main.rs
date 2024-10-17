#[cfg(feature = "snmalloc")]
#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

use clap::{Parser, Subcommand};
use hatsu_utils::AppError;
use human_panic::{metadata, setup_panic};

mod run;

#[derive(Debug, Parser)]
#[command(
    name = "hatsu",
    bin_name = "hatsu",
    version = hatsu_utils::VERSION,
    about,
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run,
}

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    setup_panic!(metadata!().homepage("https://github.com/importantimport/hatsu/issues"));

    match Args::try_parse() {
        Ok(args) => match args.command {
            Commands::Run => run::run().await,
        },
        Err(_) => run::run().await,
    }
}
