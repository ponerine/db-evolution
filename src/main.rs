use openssl::ssl::{SslConnector, SslMethod};
use postgres::Client;
use postgres_openssl::MakeTlsConnector;

extern crate postgres;

fn main() {
    println!("Making secure connection to cockroachdb");

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    // TODO pass in CA cert path
    builder.set_ca_file(ca_path).unwrap();

    let connector = MakeTlsConnector::new(builder.build());
    // TODO pass in host and port params.
    let connect_str = format!("postgresql://{}:{}@{}:{}/{}?options=--cluster%3D{}", user, password,
    host, port, db, cluster_name);
    let mut client = Client::connect( &connect_str, connector).unwrap();

    // Create the "accounts" table.
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS accounts (id INT PRIMARY KEY, balance INT)",
            &[],
        )
        .unwrap();

    // Insert two rows into the "accounts" table.
    client
        .execute(
            "INSERT INTO accounts (id, balance) VALUES (3, 2000), (4, 550)",
            &[],
        )
        .unwrap();

    // Print out the balances.
    println!("Initial balances:");
    for row in &client
        .query("SELECT id, balance FROM accounts", &[])
        .unwrap()
    {
        let id: i64 = row.get(0);
        let balance: i64 = row.get(1);
        println!("{} {}", id, balance);
    }
}
