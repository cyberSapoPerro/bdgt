use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bdgt_rs", version = "1.0", about = "Simple budget manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set your budget
    Set {
        #[arg(long)]
        budget: f64,
        #[arg(long)]
        until: String,
    },

    /// Add a new expense
    Add {
        #[arg(long)]
        name: String,

        #[arg(long)]
        price: f64,
    },
}

fn main() {
    let cli = Cli::parse();
}
