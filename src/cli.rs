use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "gfs")]
#[clap(about = "Graph Feature Store CLI", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Apply {},

    #[clap(arg_required_else_help = true)]
    Materialize {
        time: String,
    },

    Serve {},

    Clean {},
}
