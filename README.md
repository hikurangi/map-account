# Map Account

## Overview
This is a very basic Rust script which will map one style of bank account statement to another.

Specifically, this maps the content of new (c. 2025) Kiwibank account statements (CSV only) to the previous version, also as a CSV.

## Usage
1. **Clone this repo**
```bash
git clone https://github.com/hikurangi/map-account.git
```

2. **Build the app**
```bash
cargo build
```

3. **Run the script**.
It's unfinished and only runs in debug mode, so from the project root, use the following command, with your own appropriate substitutions (`output-file-name.csv`, `full-input-file-path.csv`).
```bash
./target/debug/map-account ./output-file-name.csv < full-input-file-path.csv
```
