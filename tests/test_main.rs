use std::fs::File;
use std::io::Write;
use tempdir::TempDir;

/*#[test]
fn test_load_csv_into_db() {
    // Set up mock CSV data
    let data = r#"id,name,quantity
1,Apple,100
2,Banana,150
3,Cherry,200
"#;

    let tmp_dir = TempDir::new("csv2sqlite").unwrap();
    let csv_path = tmp_dir.path().join("test.csv");
    let mut tmp_file = File::create(&csv_path).unwrap();
    tmp_file.write_all(data.as_bytes()).unwrap();

    // Load CSV data into SQLite
    Project2::load_csv_into_db(csv_path.to_str().unwrap()).unwrap();

    // TODO: Now, you would typically query the SQLite database to ensure the data was loaded correctly.
    // This can be done using the lib::execute_query function, but it currently prints the results instead of returning them.
    // Consider modifying the execute_query function to return results so they can be verified here.
}
*/
#[test]
fn test_execute_query() {
    // TODO: Set up test database data first.

    // Use the lib::execute_query to test querying the database.
    // As with the previous test, consider modifying the function to return results for verification.
}
