use mysql::*;
use mysql::prelude::*;

pub async fn list_tables() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = "mysql://root:secret@127.0.0.1:9966/test";
    let pool = Pool::new(db_url)?;

    let mut conn = pool.get_conn()?;
    let tables = conn.query_map("SHOW TABLES", |table| {
        let table: String = table;
        table
    })?;

    println!("Tables: {:?}", tables);

    Ok(())
}
