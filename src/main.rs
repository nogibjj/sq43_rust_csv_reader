use std::error::Error;

use csv;

fn csv_reader(path: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records(){
        let line = result?;
        println!("{:?}", line);
    }
    Ok(())
}

fn main() {
    if let Err(e) = csv_reader("./sample.csv"){
        eprintln!("{}", e);
    }
}