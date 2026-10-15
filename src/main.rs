use chrono::Days;
use chrono::Local;
use chrono::NaiveDate;
use clap::Parser;
use clap::Subcommand;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

// Parser

#[derive(Parser)]
#[command(
    name = "bdgt_rs",
    version = "1.0",
    about = "Simple budget manager"
)]
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

// Json managment

#[derive(Serialize, Deserialize, Debug)]
struct State {
    start: NaiveDate,
    end: NaiveDate,
    last_entry: NaiveDate,
    total_budget: f64,
    budget_left: f64,
    todays_budget: f64,
}

fn state_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap();
    PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("bdgt_rs")
        .join("state.json")
}

fn save_state(state: &State) -> Result<(), Box<dyn std::error::Error>> {
    let path = state_path();
    fs::create_dir_all(path.parent().unwrap())?;
    let json = serde_json::to_string_pretty(state)?;
    fs::write(path, json)?;
    Ok(())
}

fn load_state() -> Result<State, Box<dyn std::error::Error>> {
    let path = state_path();
    let json = fs::read_to_string(path)?;
    let state: State = serde_json::from_str(&json)?;
    Ok(state)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    match cli.command {
        Commands::Set { budget, until } => {
            let today: NaiveDate = Local::now().date_naive();
            let future: NaiveDate = if until.starts_with('+') {
                let days_left: u64 = until.parse().unwrap();
                today.checked_add_days(Days::new(days_left)).unwrap() // TODO Error managment
            } else {
                NaiveDate::parse_from_str(&until, "%Y-%m-%d").unwrap() // TODO Error managment
            };
            let days_left = (future - today).num_days() as f64;
            let state = State {
                start: today,
                end: future,
                last_entry: today,
                total_budget: budget,
                budget_left: budget,
                todays_budget: budget / days_left
            };
            save_state(&state)
        }
        
        Commands::Add { name, price } => {
            println!("Add command");
            Ok(())
        }
    }

}
