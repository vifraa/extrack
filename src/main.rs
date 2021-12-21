use std::process;

use clap::App;
use clap::Arg;
use extrack::config::Config;

fn main() {
    let args = App::new("Extrack")
        .author("Viktor Franzén, <viktor@frnzn.com>")
        .about("Extracting information from your expenses")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::new("timerange")
                .short('t')
                .long("timerange")
                .possible_value("Year")
                .possible_value("Month")
                .possible_value("Week")
                .default_value("Month")
                .takes_value(true)
        )
        .arg(
            Arg::new("input")
                .required(true)
        ).get_matches();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = extrack::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
