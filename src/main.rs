
mod audio_source;
mod constants;
mod cmd_configure;
mod cmd_start;
mod cmd_status;

use clap::{Parser, Subcommand};
use constants::*;

#[derive(Parser)]
#[command(name = "tyncan")]
#[command(about = "🎵 TynCan: Turn Your Node into a Castable Audio Network")]
#[command(version = APP_VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure audio devices for TynCan
    Configure {
        /// Skip interactive prompts and use defaults
        #[arg(short, long)]
        auto: bool,
    },
    /// Start the TynCan audio streaming service
    Start {
        /// Audio device index to use
        #[arg(short, long)]
        device: Option<i32>,
        /// Port to listen on
        #[arg(short, long, default_value_t = DEFAULT_PORT)]
        port: u16,
    },
    /// Show status of TynCan service
    Status,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Configure { auto } => {
            cmd_configure::run_configure(*auto)
        }
        Commands::Start { device, port } => {
            cmd_start::run_start(*device, *port)
        }
        Commands::Status => {
            cmd_status::run_status()
        }
    }
}


