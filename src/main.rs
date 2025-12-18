use std::{error::Error, io, process};

use csv::{Reader, Writer};

// 1. define input type ✅
// 2. define output type ✅
mod types;

// 3. ingest csv file from stdin ✅
// 3.b) optional, ingest data from file (skip for now!) https://docs.rs/csv/latest/csv/tutorial/index.html#reading-csv

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // NOTE: can use ReaderBuilder for configuration, ie:
    // specifying that the file has no headers

    // let mut rdr = csv::ReaderBuilder::new()
    //     .has_headers(false)
    //     .from_reader(io::stdin());

    // TODO: from disk?
    let mut rdr = Reader::from_reader(io::stdin());

    // TODO: to disk?
    let mut wtr = Writer::from_writer(io::stdout()); // assuming it defaults with headers

    // 5. map ✅
    for result in rdr.deserialize::<types::input::RawKiwibankInputAccount>() {
        let in_rec = result?;
        println!("{:?}", in_rec);

        let out_rec: types::output::RawKiwibankOutputAccount = in_rec.into();
        println!("{:?}", out_rec);

        // wtr.serialize(out_rec)?;
    }

    wtr.flush()?;
    Ok(())
}

// 6. output file to disk
