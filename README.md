# Rust CLI Binary with SQLite
A good starting point for a new Rust project

## CLI Usage
### To display help
```bash
cargo run -- --help
```
### To execute a SQL query
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

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
