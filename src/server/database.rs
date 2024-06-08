use std::error::Error;
use postgres::{Client, NoTls};

pub fn connect_db() -> Result<Client, Box<dyn Error>> {
    let client = Client::connect("host=localhost user=user password=password dbname=dbname", NoTls)?;
    Ok(client)
}

pub fn example_query(client: &mut Client) -> Result<(), Box<dyn Error>> {
    for row in client.query("SELECT id, name FROM example_table", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        println!("Found row: {} {}", id, name);
    }
    Ok(())
}
