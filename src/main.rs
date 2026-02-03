use std::{env, error::Error, ffi::OsString, io, process};

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

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    // NOTE: can use ReaderBuilder for configuration, ie:
    // specifying that the file has no headers

    // let mut rdr = csv::ReaderBuilder::new()
    //     .has_headers(false)
    //     .from_reader(io::stdin());

    // TODO: from disk?
    let mut rdr = Reader::from_reader(io::stdin());

    let mut wtr = Writer::from_path(file_path)?;

    // 5. map ✅
    for result in rdr.deserialize::<types::input::RawKiwibankInputAccount>() {
        let in_rec = result?;
        // println!("{:?}", in_rec);

        let out_rec: types::output::RawKiwibankOutputAccount = in_rec.try_into()?;
        // println!("{:?}", out_rec);

        // 6. output file (to disk) ✅ -> wtr.serialize just send the record to the writer's specified destination
        wtr.serialize(out_rec)?;
    }

    wtr.flush()?;
    Ok(())
}
