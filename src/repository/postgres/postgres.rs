use postgres::{Client, Error, NoTls};
use std::env;

pub fn connect_to_postgres() -> Result<Client, Error> {
    let postgres_url =
        env::var("POSTGRES_URI").expect("You must set the POSTGRES_URI environment var!");
    let connection_string = postgres_url;
    Client::connect(connection_string, NoTls)
}
