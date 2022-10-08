use sha2::{Sha256, Digest};
use openssl::ssl::{SslConnector, SslMethod};
use postgres::{Client, Error, IsolationLevel};
use postgres_openssl::MakeTlsConnector;
use std::fs;
use std::path::Path;
use chrono::prelude::*;

extern crate postgres;

// add rustc-serialize feature

// #[cfg(feature = "rustc-serialize")]
fn main() -> Result<(), Error> {
    println!("Making secure connection to cockroachdb");

    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    // TODO pass in CA cert path
    // builder.set_ca_file(ca_path).unwrap();

    let connector = MakeTlsConnector::new(builder.build());
    // TODO pass in host and port params.
    // let connect_str = format!("postgresql://{}:{}@{}:{}/{}?options=--cluster%3D{}", user, password,
    //     host, port, db, cluster_name);
    let connect_str = format!("postgresql://");
    let mut client = Client::connect( &connect_str, connector).unwrap();

    // read files in sql folder
    let sql_path = Path::new("sql");
    let sql_files = fs::read_dir(sql_path).unwrap();

    for file in sql_files {
        let mut transaction = client.build_transaction()
            .isolation_level(IsolationLevel::RepeatableRead)
            .start()?;
        let file_path = file.unwrap().path();
        let file_contents = fs::read_to_string(&file_path).unwrap();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        println!("Running SQL file: {}", file_name);
        // hash file contents with sha256 string
        let mut hasher = Sha256::new();
        hasher.update(&file_contents);
        let file_hash = format!("{:X}", hasher.finalize());
        // check if file has already been run
        let sql = "SELECT hash FROM migrations WHERE id = $1";
        let row = transaction.query_one(sql, &[&file_name]).unwrap();
        if row.len() == 0 {
            println!("Running SQL file: {}", file_name);
            let execute_result = transaction.batch_execute(&file_contents);
            let status;
            let mut failure_reason = String::new();
            
            match execute_result {
                Ok(_) => status = "success",
                Err(e) => {
                    status = "failure";
                    let error_message = e.to_string();
                    failure_reason =  error_message.clone();
                    println!("Failed to run SQL file: {}", file_name);
                },
            }
            let sql = "INSERT INTO migrations (id, hash, created_at, failure_reason, content, status) VALUES ($1, $2, $3, $4, $5, $6)";

            let reason = &failure_reason;
            transaction.execute(sql, &[&file_name, &file_hash, &Utc::now(), reason, &file_contents, &status]).unwrap();
            println!("Finished SQL file {} execution.", file_name);
            if failure_reason.len() > 0 {
                transaction.rollback()?;
                break;
            } else {
                transaction.commit()?;
            }
        } else {
            // check if hash is the same
            let hash: &str = row.get(0);
            if hash != file_hash {
                println!("Hashes do not match for file: {}. Break.", file_name);
                transaction.rollback()?;
                break;

            }
            println!("SQL file: {} has been executed. Next.", file_name);
        }
    }
    Ok(())
}
