
use calamine::{open_workbook, DataType, RangeDeserializerBuilder, Reader, Xlsx};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::{collections::HashMap, error::Error, vec};
use std::io;
use chrono::NaiveDate;

use config::{Config, TimeRange};


pub mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let parsed_rows = parse_workbook(&config)?;
    let grouped_transactions = group_transactions(&config, parsed_rows);
    let summaries = calculate_summaries(grouped_transactions);

    match config.output_path {
        Some(path) => write_to_file(summaries, &path)?,
        None => write_to_stdout(summaries)?,
    };

    Ok(())
}

fn group_transactions(config: &Config, rows: Vec<Transaction>) -> HashMap<String, Vec<Transaction>> {
    let mut grouped: HashMap<String, Vec<Transaction>> = HashMap::new();

    for row in rows {
        // TODO should probably do something more dynamic than a hardcoded date format.
        // Should also fix the unwrap.
        let date = NaiveDate::parse_from_str(&row.date, "%Y-%m-%d").unwrap();

        let map_key = match config.time_range {
            TimeRange::Year => date.format("%Y").to_string(),
            TimeRange::Month => date.format("%Y-%m").to_string(),
            TimeRange::Week => date.format("%Y-%W").to_string(),
        };
        let current = grouped.entry(map_key).or_insert_with(Vec::new);
        current.push(row);
    }

    grouped
}

fn write_to_stdout(summaries: Vec<Summary>) -> Result<(), Box<dyn Error>> {
    let mut header: Vec<String> = summaries.iter()
        .map(|s| s.category_breakdown.keys().cloned().collect::<Vec<String>>())
        .flatten()
        .collect();
    header.sort();
    header.dedup();
    header.insert(0, String::from("Date"));

    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&header)?;

    for summary in summaries {
        let mut row: Vec<String> = vec![summary.date];

        // Skip first one since that is the date
        for h in header.iter().skip(1) {
            let value = summary.category_breakdown.get(h).unwrap_or(&0.0);
            row.push(value.to_string());
        }
        writer.write_record(row)?;
    }

    writer.flush()?;
    Ok(())
}

fn write_to_file(summaries: Vec<Summary>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut header: Vec<String> = summaries.iter()
        .map(|s| s.category_breakdown.keys().cloned().collect::<Vec<String>>())
        .flatten()
        .collect();
    header.sort();
    header.dedup();
    header.insert(0, String::from("Date"));

    let mut writer = csv::Writer::from_path(file_path)?;
    writer.write_record(&header)?;

    for summary in summaries {
        let mut row: Vec<String> = vec![summary.date];
        // Skip first one since that is the date
        for h in header.iter().skip(1) {
            let value = summary.category_breakdown.get(h).unwrap_or(&0.0);
            row.push(value.to_string());
        }
        writer.write_record(row)?;
    }

    writer.flush()?;
    Ok(())
}

fn calculate_summaries(date_grouped_transactions: HashMap<String, Vec<Transaction>>) -> Vec<Summary> {
    let mut found_summaries: Vec<Summary> = Vec::new();
    for (date, transactions) in date_grouped_transactions.iter() {
        let mut total = 0.0;
        let mut category_grouped: HashMap<String, f64> = HashMap::new();
        for transaction in transactions {
            // TODO Creating new strings like is done below here cant be the correct way to do this.
            // It works for now but really needs to be fixed.
            let value = match category_grouped.entry(String::from(&transaction.category)) {
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

        found_summaries.push(
            Summary {
                date: date.to_string(),
                income,
                expenses,
                total,
                category_breakdown: category_grouped,
            }
        );
    }
    found_summaries.sort_by(|a,b| a.date.cmp(&b.date));
    found_summaries
}

#[derive(Debug, Serialize, Deserialize)]
struct Summary {
    date: String,
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
