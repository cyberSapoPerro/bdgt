use chrono::Days;
use chrono::Local;
use chrono::NaiveDate;
use clap::Parser;
use clap::Subcommand;
use colored::Colorize;
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

// CSV managment

#[derive(Serialize, Deserialize, Debug)]
struct Entry{
    date: NaiveDate,
    name: String,
    price: f64,
}

fn save_entry(entry: &Entry) -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME").unwrap();
    let path = PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("bdgt_rs")
        .join("db.csv");
    fs::create_dir_all(path.parent().unwrap())?;
    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(!path.exists())
        .from_writer(file);
    writer.serialize(entry)?;
    writer.flush()?;
    Ok(())
}

fn print_budget() -> Result<(), Box<dyn std::error::Error>>{
    let mut state = load_state()?;
    let today = Local::now().date_naive();
    let days_left = state.end.signed_duration_since(today).num_days();

    if state.last_entry < today {
        if days_left > 0 {
            state.todays_budget = state.budget_left / days_left as f64;
            save_state(&state)?;
        }
    } 

    // Print
    let line1 = format!(
        "{} {} {:.2}",
        "Todays budget:".bold(),
        "$".green(),
        state.todays_budget.to_string().green()
    );
    let line2 = format!(
        "{} {:.2} {} {} {}",
        "Budget left: $".dimmed(),
        state.budget_left.to_string().dimmed(),
        "for".dimmed(),
        days_left.to_string().dimmed(),
        "days".dimmed(),
    );

    let line1_width = console::measure_text_width(&line1);
    let line2_width = console::measure_text_width(&line2);
    let content_width = line1_width.max(line2_width);
    let width = content_width + 2;

    let top = format!("╭{}╮", "─".repeat(width));
    let bottom = format!("╰{}╯", "─".repeat(width));

    println!("{}", top);
    println!("│ {}{} │", line1, " ".repeat(content_width - line1_width));
    println!("│ {}{} │", line2, " ".repeat(content_width - line2_width));
    println!("{}", bottom);

    Ok(())
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
            save_state(&state)?;

            print_budget()?;

            Ok(())
        }
        
        Commands::Add { name, price } => {
            let today: NaiveDate = Local::now().date_naive();
            let entry = Entry {
                date: today,
                name: name,
                price: price
            };
            save_entry(&entry)?;

            let mut state = load_state().unwrap();
            state.budget_left -= price;
            state.todays_budget -= price;
            save_state(&state)?;

            print_budget()?;

            Ok(())
        }
    }

}
