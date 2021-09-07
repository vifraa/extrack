use std::{collections::HashMap, error::Error, vec};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use calamine::{RangeDeserializerBuilder, Reader, Xlsx, open_workbook, DataType};

pub struct Config {
    pub file_path: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let parsed_rows = parse_workbook(&config)?;

    let mut expenses_grouped: HashMap<String, f64> = HashMap::new();
    let mut income_grouped: HashMap<String, f64> = HashMap::new();
    
    for row in parsed_rows {
        if row.amount > 0.0 {
            let value = match income_grouped.entry(row.category) {
                Vacant(entry) => entry.insert(0.0),
                Occupied(entry) => entry.into_mut(),
            };
            *value += row.amount;

        } else {
            let value = match expenses_grouped.entry(row.category) {
                Vacant(entry) => entry.insert(0.0),
                Occupied(entry) => entry.into_mut(),
            };
            *value += row.amount;
        }
    }


    println!("Income: ");
    income_grouped.iter().for_each(|f| println!("{}, {}", f.0, f.1));
    println!("\n Expenses: ");
    expenses_grouped.iter().for_each(|f| println!("{}, {}", f.0, f.1));

    Ok(())
}


#[derive(Debug)]
struct Transaction {
    date: String,
    description: String,
    amount: f64,
    category: String
}

impl Transaction {
    fn parse_from_excel_row(row: Vec<DataType>) -> Result<Transaction, String> {
        let transaction = Transaction {
            date: row[1].to_string(),
            description: row[2].to_string(),
            amount: row[3].get_float().unwrap_or(0.0),
            category: row[5].get_string().unwrap_or("Unspecified").to_string()
        };

        if transaction.amount == 0.0 {
            return Err(format!("Error parsing row: date: {}, description: {}, amount: {}, category: {}", 
                        transaction.date, transaction.description, transaction.amount, transaction.category))
        }

        Ok(transaction)
    }
}



fn parse_workbook(config: &Config) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut workbook: Xlsx<_> = open_workbook(&config.file_path)?;

    // TODO Remove the clone here.
    let range = workbook.worksheets()
        .first()
        .ok_or(calamine::Error::Msg("could not find a sheet in given excel"))?.clone();


    let mut result = vec![];
    let mut iter_result = RangeDeserializerBuilder::new().from_range(&range.1)?;
    while let Some(r) = iter_result.next() {
        let row: Vec<DataType> = r?;
        
        let transaction = Transaction::parse_from_excel_row(row);
        match transaction {
            Ok(t) => result.push(t),
            Err(t) => println!("{}", t)
        }
    }

    Ok(result)
}



