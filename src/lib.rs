use calamine::{open_workbook, DataType, RangeDeserializerBuilder, Reader, Xlsx};
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::{collections::HashMap, error::Error, vec};
use std::{env, io};

pub struct Config {
    pub file_path: String,
    pub output_path: Option<String>,
    pub date_column: usize,
    pub description_column: usize,
    pub amount_column: usize,
    pub category_column: usize,
    pub first_row_index: usize,
}

impl Config {
    pub fn new(args: &ArgMatches) -> Result<Config, &str> {
        // TODO fix all of these unwraps, should handle in a good way with the Result return type.
        let file_path = args.value_of("input").unwrap();
        let output_path = match args.value_of("output") {
            Some(v) => Some(v.to_owned()),
            None => None,
        };

        let date_column: usize = env::var("EXTRACK_DATE_COLUMN")
            .unwrap_or(String::from("0"))
            .parse()
            .unwrap_or(0);
        let description_column: usize = env::var("EXTRACK_DESCRIPTION_COLUMN")
            .unwrap_or(String::from("1"))
            .parse()
            .unwrap_or(1);
        let amount_column: usize = env::var("EXTRACK_AMOUNT_COLUMN")
            .unwrap_or(String::from("2"))
            .parse()
            .unwrap_or(2);
        let category_column: usize = env::var("EXTRACK_CATEGORY_COLUMN")
            .unwrap_or(String::from("3"))
            .parse()
            .unwrap_or(3);

        let first_row_index: usize = env::var("EXTRACK_FIRST_ROW_INDEX")
            .unwrap_or(String::from("0"))
            .parse()
            .unwrap_or(0);

        Ok(Config {
            file_path: file_path.to_string(),
            output_path,
            date_column,
            description_column,
            amount_column,
            category_column,
            first_row_index,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let parsed_rows = parse_workbook(&config)?;
    let summary = calculate_summary(parsed_rows);

    match config.output_path {
        Some(path) => write_to_file(summary, &path)?,
        None => write_to_stdout(summary)?,
    };

    // TODO should give option between json and csv outputs
    //println!("{}", serde_json::to_string_pretty(&summary).unwrap());

    Ok(())
}

fn write_to_stdout(summary: Summary) -> Result<(), Box<dyn Error>> {
    let mut header: Vec<&str> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    for (key, value) in summary.category_breakdown.iter() {
        header.push(key);
        row.push(value.to_string());
    }

    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(header)?;
    writer.write_record(row)?;
    writer.flush()?;
    Ok(())
}

fn write_to_file(summary: Summary, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut header: Vec<&str> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    for (key, value) in summary.category_breakdown.iter() {
        header.push(key);
        row.push(value.to_string());
    }

    let mut writer = csv::Writer::from_path(file_path)?;
    writer.write_record(header)?;
    writer.write_record(row)?;
    writer.flush()?;
    Ok(())
}

fn calculate_summary(transactions: Vec<Transaction>) -> Summary {
    let mut total = 0.0;
    let mut category_grouped: HashMap<String, f64> = HashMap::new();
    for transaction in transactions {
        let value = match category_grouped.entry(transaction.category) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(0.0),
        };
        *value += transaction.amount;
        total += transaction.amount;
    }

    let mut expenses = 0.0;
    let mut income = 0.0;
    category_grouped.iter().for_each(|f| {
        if f.1 > &0.0 {
            income += f.1;
        } else {
            expenses += f.1;
        }
    });

    Summary {
        income,
        expenses,
        total,
        category_breakdown: category_grouped,
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Summary {
    income: f64,
    expenses: f64,
    total: f64,
    category_breakdown: HashMap<String, f64>,
}

#[derive(Debug)]
struct Transaction {
    date: String,
    description: String,
    amount: f64,
    category: String,
}

impl Transaction {
    fn parse_from_excel_row(row: Vec<DataType>, config: &Config) -> Result<Transaction, String> {
        let transaction = Transaction {
            date: row[config.date_column].to_string(),
            description: row[config.description_column].to_string(),
            amount: row[config.amount_column].get_float().unwrap_or(0.0),
            category: row[config.category_column]
                .get_string()
                .unwrap_or("Unspecified")
                .to_string(),
        };

        if transaction.amount == 0.0 {
            return Err(format!(
                "Error parsing row: date: {}, description: {}, amount: {}, category: {}",
                transaction.date, transaction.description, transaction.amount, transaction.category
            ));
        }

        Ok(transaction)
    }
}

fn parse_workbook(config: &Config) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut workbook: Xlsx<_> = open_workbook(&config.file_path)?;

    // TODO Remove the clone here.
    let range = workbook
        .worksheets()
        .first()
        .ok_or(calamine::Error::Msg(
            "could not find a sheet in given excel",
        ))?
        .clone();

    let mut result = vec![];
    let mut iter_result = RangeDeserializerBuilder::new()
        .from_range(&range.1)?
        .skip(config.first_row_index);
    while let Some(r) = iter_result.next() {
        let row: Vec<DataType> = r?;

        let transaction = Transaction::parse_from_excel_row(row, config);
        match transaction {
            Ok(t) => result.push(t),
            Err(t) => println!("{}", t),
        }
    }

    Ok(result)
}
