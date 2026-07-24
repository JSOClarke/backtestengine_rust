use std::{error::Error, num::ParseFloatError, string::FromUtf8Error};

#[cfg(test)]
#[test]

fn test_dataloader() {}

#[derive(Debug)]
pub struct Candle {
    date: String,
    close: f64,
    volume: f64,
    open: f64,
    high: f64,
    low: f64,
}
// down the line turn this into a enum with the errors then you have to use the from trait to turn the errors into dataloader
// enum DataLoaderError {
//     InvalidUtf8(std::string::FromUtf8Error),
//     InvalidNumber(std::num::ParseFloatError),
// }
// This will convert the csv format historical data in array of candles

pub fn dataloader(data: &[u8]) -> Result<Vec<Candle>, Box<dyn Error>> {
    let mut field_data: Vec<Vec<u8>> = vec![];
    let mut field: Vec<u8> = vec![];
    let mut candle_array: Vec<Candle> = vec![];
    //  let candle:Candle;
    // maybe have a delimter at the comma
    let mut is_header_row: bool = true;

    for byte in data {
        if is_header_row {
            if *byte == 10 {
                is_header_row = false;
            }
            continue;
        }

        //comma split
        if *byte == 44 {
            field_data.push(field.clone());
            field.clear();
            continue;
        }
        // dollar currency sign
        if *byte == 36 {
            continue;
        }
        // newline split
        if *byte == 13 || *byte == 10 {
            field_data.push(field.clone());
            field.clear();
            let field_0: String = String::from_utf8(field_data[0].clone())?; // this field needs to be u64

            let field_1: f64 = String::from_utf8(field_data[1].clone())?.parse()?;
            let field_2: f64 = String::from_utf8(field_data[2].clone())?.parse()?; // this field needs to be u64
            let field_3: f64 = String::from_utf8(field_data[3].clone())?.parse()?; // this field needs to be u64
            let field_4: f64 = String::from_utf8(field_data[4].clone())?.parse()?; // this field needs to be u64
            let field_5: f64 = String::from_utf8(field_data[5].clone())?.parse()?; // this field needs to be u64
            // dbg!(&field_5);

            let candle: Candle = Candle {
                date: field_0.clone(),
                close: field_1.clone(),
                volume: field_2.clone(),
                open: field_3.clone(),
                high: field_4.clone(),
                low: field_5.clone(),
            };
            candle_array.push(candle);
            continue;
        }
        field.push(*byte);
    }

    Ok(candle_array)
}
