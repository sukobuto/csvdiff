use clap::{App, ArgMatches};
use std::error::Error;
use std::process;

fn get_args() -> ArgMatches {
    App::new("csvdiff")
        .version("0.1")
        .author("Kenta Suzuki <k-suzuki@mi-labo.co.jp>")
        .about("csv diff tool")
        .arg("<file_a> 'Sets file A.'")
        .arg("<file_b> 'Sets file B.'")
        .get_matches()
}

fn diff(file_a: &str, file_b: &str) -> Result<(), Box<dyn Error>> {
    let mut rd_a = csv::Reader::from_path(file_a)?;
    let mut rd_b = csv::Reader::from_path(file_b)?;
    for (row, result) in rd_a.records().zip(rd_b.records()).enumerate() {
        if let (Ok(record_a), Ok(record_b)) = result {
            if record_a != record_b {
                for (col, (a, b)) in record_a.iter().zip(record_b.iter()).enumerate() {
                    if a != b {
                        println!("{},{},\"{}\",\"{}\"", row, col, a, b);
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let args = get_args();
    let file_a = args.value_of("file_a").unwrap();
    let file_b = args.value_of("file_b").unwrap();
    
    if let Err(err) = diff(&file_a, &file_b) {
        println!("error running diff: {}", err);
        process::exit(1);
    }
}
