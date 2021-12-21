use std::env;

use clap::ArgMatches;

#[derive(Debug)]
pub enum TimeRange {
    Year,
    Month,
    Week
}

pub struct Config {
    pub file_path: String,
    pub time_range: TimeRange,
    pub output_path: Option<String>,
    pub date_column: usize,
    pub description_column: usize,
    pub amount_column: usize,
    pub category_column: usize,
    pub first_row_index: usize,
}

impl Config {
    pub fn new(args: &ArgMatches) -> Result<Config, &str> {
        let file_path = match args.value_of("input") {
            Some(input) => input,
            None => return Err("could not find input argument")

        };
        let output_path = args.value_of("output").map(|v| v.to_owned());
        let time_range = match args.value_of("timerange").unwrap() {
            "Year" => TimeRange::Year,
            "Month" => TimeRange::Month,
            "Week" => TimeRange::Week,
            _ => return Err("did not receive a valid timerange argument")
        };

        let date_column: usize = env::var("EXTRACK_DATE_COLUMN")
            .unwrap_or_else(|_| String::from("0"))
            .parse()
            .unwrap_or(0);
        let description_column: usize = env::var("EXTRACK_DESCRIPTION_COLUMN")
            .unwrap_or_else(|_| String::from("1"))
            .parse()
            .unwrap_or(1);
        let amount_column: usize = env::var("EXTRACK_AMOUNT_COLUMN")
            .unwrap_or_else(|_| String::from("2"))
            .parse()
            .unwrap_or(2);
        let category_column: usize = env::var("EXTRACK_CATEGORY_COLUMN")
            .unwrap_or_else(|_| String::from("3"))
            .parse()
            .unwrap_or(3);

        let first_row_index: usize = env::var("EXTRACK_FIRST_ROW_INDEX")
            .unwrap_or_else(|_| String::from("0"))
            .parse()
            .unwrap_or(0);

        Ok(Config {
            file_path: file_path.to_string(),
            time_range,
            output_path,
            date_column,
            description_column,
            amount_column,
            category_column,
            first_row_index,
        })
    }
}
