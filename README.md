[![CI](https://github.com/nogibjj/python-template/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/python-template/actions/workflows/cicd.yml)
# Rust CLI Binary with SQLite
- SQLite Database: Include a SQLite database and demonstrate CRUD (Create, Read, Update, Delete) operations.
- Optimized Rust Binary: Include a process that generates an optimized Rust binary as a GitHub Actions artifact that can be downloaded.
- GitHub Actions: A workflow file that tests, builds, and lints your Rust code.
- Video Demo: A YouTube link in README.md showing a clear, concise walkthrough and demonstration of your CLI binary.

## CLI Usage
### To display help
```bash
cargo run -- --help
```
### To execute a SQL query in SQLite
```bash
cargo run -- query "SELECT * FROM product"
```
### To insert a new record
```bash
cargo run -- query "INSERT INTO product (id, name, quantity) VALUES (4, 'Orange', 40)"
```
### To update a product's quantity
```bash
cargo run -- query "UPDATE product SET quantity = 50 WHERE id = 4"
```
### To delete a product by id
```bash
cargo run -- query "DELETE FROM product WHERE id = 4"
```
### To load data from a CSV file into the SQLite database
```bash
cargo run -- load "sample.csv"
```
## Result
### CLI help
![help](Result/help.png)
### Query Demo
![insert1](Result/insert1.png)
![insert2](Result/insert2.png)
## Test
![test](Result/test.png)
## Optimized Rust Binary
![artifact](Result/binary.png)
## YouTube Link
https://youtu.be/LBiJOxeJfW4
## References
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
