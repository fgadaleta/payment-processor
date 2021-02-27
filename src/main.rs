use payment_processor::bank;
use payment_processor::transaction::Tx;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io;
use serde::Serialize;


#[derive(Debug, Serialize)]
    struct Record {
        client: u16,
        available: f32,
        held: f32,
        total: f32,
        locked: bool,
    }

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // println!("Welcome to TxProcessor!");
    
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    
    // initialize bank
    let mut processor = bank::Bank::init();

    for result in rdr.deserialize::<Tx>() {
        let transaction = result?;
        // println!("Processing transaction {:?}", &transaction);
        let _res = processor.process(&transaction);
    }

    let accounts = processor.get_accounts();
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for (client_id, acc) in accounts.iter() {
        wtr.serialize(Record {
            client: client_id.to_owned(),
            available: acc.available,
            held: acc.held,
            total: acc.total,
            locked: acc.locked
        })?;  
    }
    Ok(())
}
