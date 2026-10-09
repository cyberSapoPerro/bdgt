import argparse
import json
import os
from pathlib import Path
from datetime import datetime, timedelta
from textwrap import indent

import pandas as pd
from rich.console import Console
from rich.panel import Panel
from rich.table import Table
from rich.text import Text


DIR = Path.home() / ".local" / "share" / "bdgt"
JSON_PATH = DIR / "state.json"
DF_PATH = DIR / "df.csv"
CONSOLE = Console()


def setup_path():
    DIR.mkdir(exist_ok=True, parents=True)


def load_state() -> dict:
    if JSON_PATH.exists():
        with JSON_PATH.open("r", encoding="utf-8") as f:
            return json.load(f)
    state = {}
    with JSON_PATH.open("w", encoding="utf-8") as f:
        json.dump(state, f, indent=4)
    return state


def save_state(state: dict):
    with JSON_PATH.open("w", encoding="utf-8") as f:
        json.dump(state, f, indent=4)


def load_df() -> pd.DataFrame:
    if DF_PATH.exists():
        return pd.read_csv(DF_PATH)
    else:
        return pd.DataFrame(columns = ["date", "name", "price"])


def process_dates_for_setup(start, end):
    if not start:
        start = datetime.now()
    else:
        start = datetime.strptime(start, "%d/%m/%Y")
    if end[0] == "+":
        end = start + timedelta(days=int(end[1:]))
    else:
        end = datetime.strptime(end, "%d/%m/%Y")
    return (start, end)


def calculate_days_left(start: str, end: str) -> int:
    start_dt = datetime.strptime(start, "%d/%m/%Y")
    end_dt = datetime.strptime(end, "%d/%m/%Y")
    days_left = (end_dt - start_dt).days
    return int(days_left)


def add_entry(entry: list):
    name, price = entry
    date = datetime.now().strftime("%d/%m/%Y")
    # State operations
    state = load_state()
    state["budget_left"] = float(state["budget_left"]) - float(price)
    state["todays_budget"] = float(state["todays_budget"]) - float(price)
    state["last_entry"] = date
    save_state(state)
    # DF operations
    df = load_df()
    df.loc[len(df)] = [date, name, price]
    df.to_csv(DF_PATH, index=False)


def print_todays_budget():
    state = load_state()
    last_entry = datetime.strptime(state["last_entry"], "%d/%m/%Y")
    if last_entry < datetime.today():
        todays_budget = float(state["todays_budget"])
    else:
        budget_left = float(state["budget_left"])
        start = state["start"]
        end = state["end"]
        days_left = calculate_days_left(start, end)
        todays_budget = budget_left / days_left

    # Tabla con información secundaria
    table = Table.grid(padding=(0,2))
    table.add_column(style="dim")
    table.add_column(justify="right", style="white")
    table.add_row(
        "Presupuesto restante",
        f"$ {float(state["budget_left"]):,.2f}"
    )

    # Panel principal
    content = Table.grid(padding=(0,1))
    content.add_row(
        Text("Disponible para hoy", style="bold dim")
    )
    content.add_row(
        Text(f"$ {todays_budget:,.2}", style="bold green")
    )
    content.add_row("")
    content.add_row(table)

    # Print
    CONSOLE.print(
        Panel(
            content,
            title="[bold cyan]Tu presupuesto[/]",
            subtitle=f"[dim]{datetime.today().strftime("%d/%m/%Y")}",
            border_style="green",
            padding=(1,3),
        )
    )


def main():
    parser = argparse.ArgumentParser(
        prog="bdgt",
        description="A simple budget manager"
    )

    subparsers = parser.add_subparsers(
        dest="command",
        help="Available commands",
    )

    set_parser = subparsers.add_parser(
        "set",
        help="Set up or reset your budget",
        description=(
            "Configure your budget for a date range.\n"
            "The budget is distributed evenly across the selected days."
        )
    )
    set_parser.add_argument(
        "--start",
        "-s",
        help="Start date (format: DD/MM/YYYY). Default: today.",
    )
    set_parser.add_argument(
        "--end",
        "-e",
        help="End date (format: DD/MM/YYYY or +N where N is the number of days of the interval).",
    )
    set_parser.add_argument(
        "--budget",
        "-b",
        type=float,
        required=True,
        help="Total budget for the selected period."
    )
    # TODO
    # set_parser.add_argument("-m", "--mode")

    add_parser = subparsers.add_parser(
        "add",
        help="Record a new expense",
        description="Record an expense and display your updated daily budget."
    )
    add_parser.add_argument(
        "--name",
        "-n",
        required=True,
        help="Name or description of the expense."
    )
    add_parser.add_argument(
        "--price",
        "-p",
        type=float,
        required=True,
        help="Cost of the expense."
    )

    # remove_parser = subparsers.add_parser("remove")
    # TODO

    args = parser.parse_args()

    if args.command == "set":
        setup_path()

        # Process dates
        start, end = process_dates_for_setup(args.start, args.end)
        start = start.strftime("%d/%m/%Y")
        end = end.strftime("%d/%m/%Y")

        days_left = calculate_days_left(start, end)

        # Generating state
        state = load_state()
        state = {
            "start": start,
            "end": end,
            "total_budget": args.budget,
            "budget_left": args.budget,
            "todays_budget": float(args.budget) / float(days_left),
            "last_entry": datetime.today().strftime("%d/%m/%Y"),
        }
        save_state(state)

        print("Se ha configurado el presupuesto!")
        print_todays_budget()
        
    elif args.command == "add":
        entry = [args.name, args.price]
        add_entry(entry)
        print_todays_budget()

    else:
        print_todays_budget()


if __name__ == "__main__":
    main()
