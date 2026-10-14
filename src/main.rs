use chrono::NaiveDate;
use chrono::ParseError;
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

fn parse_date(s: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

#[derive(Subcommand)]
enum Commands {
    /// Set your budget
    Set {
        #[arg(long)]
        budget: f64,
        #[arg(long, value_parser = parse_date)]
        until: NaiveDate,
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

fn main() {
    let cli = Cli::parse();
}
