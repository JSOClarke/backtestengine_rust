mod data;

mod loader;
mod format;
use std::error::Error;

use crate::loader::loader::{FileType, data_loader};


fn main() -> Result<(), Box<dyn Error>> {
    let ft = FileType::Csv;
    let path = String::from("test_1.csv"); 
    let candles = data_loader( path,ft)?;
    println!("{:?}", candles);
    Ok(())
}
