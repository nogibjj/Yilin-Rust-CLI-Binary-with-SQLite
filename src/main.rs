use clap::{App, Arg, SubCommand};
use crate::lib::{execute_query, load_csv_into_db};

mod lib;

fn main() -> rusqlite::Result<()> {
    let matches = App::new("Project2")
        .version("1.0")
        .author("Yilin Yang")
        .about("Performs CRUD operations on an SQLite database using command line SQL queries.")
        .subcommand(SubCommand::with_name("query")
            .about("Execute a raw SQL query")
            .arg(Arg::with_name("SQL")
                .help("The raw SQL query to execute")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("load")
            .about("Load data from a CSV into the SQLite database")
            .arg(Arg::with_name("CSV_PATH")
                .help("Path to the CSV file to load data from")
                .required(true)
                .index(1)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("query") {
        let query = matches.value_of("SQL").unwrap();
        execute_query(query)?;
    } else if let Some(matches) = matches.subcommand_matches("load") {
        let csv_path = matches.value_of("CSV_PATH").unwrap();
        load_csv_into_db(csv_path)?;
    }

    Ok(())
}
