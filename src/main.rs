mod fix_dates;

use clap::Parser;
use std::io;
use chrono::{NaiveDate, Local};
use fix_dates::*;

#[derive(Parser)]
struct Args {
    #[arg(short, long, value_name = "min-date", default_value_t = String::from("today"))]
    min_date: String,
}

fn main() {
    let args = Args::parse();

    let min_date = if args.min_date == "today" {
        Local::now().date_naive()
    } else {
        NaiveDate::parse_from_str(&args.min_date, "%Y-%m-%d").unwrap()
    }.format("%Y-%m-%d").to_string().as_bytes().to_vec();

    fix_dates(&mut io::stdin(), &mut io::stdout(), &min_date);
}
