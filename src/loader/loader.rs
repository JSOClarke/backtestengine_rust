use std::{error::Error, fs::File};

#[derive(PartialEq, Debug)]
pub enum FileType {
    Csv,
    Txt,
    Unknown,
    Jpeg,
    Png,
}

use crate::format::csv::loader::handle_csv;
use crate::data::candle::Candle;

// use crate::{data::candle::Candle, format::csv::loader::{self, handle_csv}};

pub fn data_loader(path:String,ft: FileType)->Result<Vec<Candle>,Box<dyn Error>>{
    let mut file = File::open(&path)?;
    
    let mut candles:Vec<Candle> = vec![];
    match ft {
        FileType::Csv => {
            candles = handle_csv(&mut file)?;
            // println!("{:?}", candles)
        }
        _ => return Err("Unsupported File Type".into()),
    }
    Ok(candles)
}


// pub fn file_loader(user_string: String, ft: FileType) -> Result<Fi, Box<dyn Error>> {
//     let mut result: Vec<Candle> = vec![];

//     Ok(())
// }