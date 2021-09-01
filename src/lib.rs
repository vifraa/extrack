use std::error::Error;
use calamine::{RangeDeserializerBuilder, Reader, Xlsx, open_workbook};

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
    println!("Given path: {}", config.file_path);
    let mut workbook: Xlsx<_> = open_workbook(config.file_path)?;
    // TODO Dont hardcode sheet name
    let range = workbook.worksheet_range("Kontoutdrag")
        .ok_or(calamine::Error::Msg("Cannot find 'Kontoutdrag'"))??;


    let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;

    while let Some(result) = iter.next() {
        let (label, value): (String, String) = result?;
        println!("Label: {}, Value: {}", label, value);

    }



    Ok(())
}


