use csv::Reader;
use rusqlite::{params, Connection, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: i32,
    pub name: String,
    pub quantity: i32,
}

pub fn execute_query(query: &str) -> Result<()> {
    let conn = Connection::open("products.db")?;

    if query.to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ))
        })?;

        for row in rows {
            let (id, name, quantity): (i32, String, i32) = row?;
            println!("{} | {} | {}", id, name, quantity);
        }
    } else {
        conn.execute(query, [])?;
    }

    Ok(())
}

pub fn load_csv_into_db(csv_path: &str) -> Result<()> {
    let conn = Connection::open("products.db")?;
    let mut rdr = Reader::from_path(csv_path).unwrap();

    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        conn.execute(
            "INSERT INTO product (id, name, quantity) VALUES (?1, ?2, ?3)",
            params![record.id, record.name, record.quantity],
        )?;
    }

    Ok(())
}
