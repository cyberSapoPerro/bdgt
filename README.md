# bdgt

A simple command-line budget manager written in Python.

Track your daily budget and record expenses directly from your terminal.

## Features

* Set a budget for a date range.
* Calculate your daily spending budget.
* Record expenses.
* Check your remaining daily budget.

## Installation

Install `bdgt` using [uv](https://docs.astral.sh/uv/):

```bash
uv tool install git+https://github.com/cyberSapoPerro/bdgt
```

To update the installation:

```bash
uv tool upgrade bdgt
```

## Usage

### Check today's budget

```bash
bdgt
```

### Set a budget

```bash
bdgt set --budget 500000 --start 09/10/2026 --end 31/10/2026
```

| Option           | Description               |
| ---------------- | ------------------------- |
| `-b`, `--budget` | Total budget              |
| `-s`, `--start`  | Start date (`DD/MM/YYYY`) |
| `-e`, `--end`    | End date (`DD/MM/YYYY`)   |

### Add an expense

```bash
bdgt add --name "Lunch" --price 15000
```

| Option          | Description    |
| --------------- | -------------- |
| `-n`, `--name`  | Expense name   |
| `-p`, `--price` | Expense amount |

## Help

Display available commands and options:

```bash
bdgt --help
bdgt set --help
bdgt add --help
```

## License

MIT
