mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{apply, clean, materialize};

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Apply {} => match apply().await {
            Ok(_) => {
                println!("Apply: Success");
            }
            Err(e) => {
                println!("Apply: Error: {}", e);
            }
        },
        Commands::Materialize { time } => match materialize().await {
            Ok(_) => {
                println!("Materialize: Success at {}", time);
            }
            Err(e) => {
                println!("Materialize: Error: {}", e);
            }
        },
        Commands::Serve {} => {
            println!("Serve");
        }
        Commands::Clean {} => match clean().await {
            Ok(_) => {
                println!("Clean: Success");
            }
            Err(e) => {
                println!("Clean: Error: {}", e);
            }
        },
    }
}
