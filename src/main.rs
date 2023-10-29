use clap::{App, Arg};
use rusqlite::Connection;
use std::process;
use YILIN_RUST_CLI_BINARY_WITH_SQLITE::load;
use YILIN_RUST_CLI_BINARY_WITH_SQLITE::transform;
use YILIN_RUST_CLI_BINARY_WITH_SQLITE::extract;

fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("You <your_email@example.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Sets the input CSV file")
            .takes_value(true))
        .arg(Arg::with_name("db")
            .short('d')
            .long("db")
            .value_name("DATABASE")
            .help("Sets the database file")
            .takes_value(true))
        .get_matches();

    let input_file = matches.value_of("input").unwrap_or("input.csv");
    let db_file = matches.value_of("db").unwrap_or("output.db");

    println!("Using input file: {}", input_file);
    println!("Using database file: {}", db_file);

    let mut conn = Connection::open(db_file).unwrap_or_else(|err| {
        eprintln!("Problem opening the database: {:?}", err);
        process::exit(1);
    });

    // Extract
    let extracted_data = extract(input_file).unwrap_or_else(|err| {
        eprintln!("Problem extracting data from file: {:?}", err);
        process::exit(1);
    });

    // Transform
    let transformed_data = transform(&extracted_data);

    // Load
    load(&mut conn, &transformed_data).unwrap_or_else(|err| {
        eprintln!("Problem loading data into the database: {:?}", err);
        process::exit(1);
    });

    println!("Data loaded successfully.");
}
