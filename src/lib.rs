// pub mod utilities;

use std::{
    error::Error, fs::File, io::Read, num::ParseFloatError, path::Path, string::FromUtf8Error,
};

pub mod utilities;

use crate::utilities::magic_number::FileType;
use crate::utilities::magic_number::detect_magic_number;

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

pub fn file_loader(user_string: String, ft: FileType) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(&user_string)?;
    let mut ft_buffer = [0u8; 16];
    file.read_exact(&mut ft_buffer)?;
    let mut result: Vec<Candle> = vec![];

    // let file_type = detect_magic_number(&ft_buffer);
    match ft {
        FileType::Csv => {
            result = handle_csv(&mut file)?;
            println!("{:?}", result)
        }
        _ => return Err("Unsupported File Type".into()),
    }
    Ok(())
}
#[cfg(test)]
#[test]

fn test_handle_csv() {
    let mut file = File::open("test_data/test_data_file_1.csv").unwrap();
    assert!(handle_csv(&mut file).is_ok());
}
pub fn handle_csv(file: &mut File) -> Result<Vec<Candle>, Box<dyn Error>> {
    let mut buffer = [0u8; 16];
    let mut fields: Vec<Vec<u8>> = vec![];
    let mut field: Vec<u8> = vec![];
    let mut candles: Vec<Candle> = vec![];
    let mut is_header: bool = true;

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        // Question - why does the buffer need to be a borrow here for chunk
        let chunk = &buffer[..bytes_read];
        parse_csv(
            &chunk,
            &mut fields,
            &mut field,
            &mut candles,
            &mut is_header,
        )?;
    }
    Ok(candles)
}

// Check that the header number count matches what the parser needs.
pub fn check_header() {}

pub fn parse_csv(
    chunk: &[u8],
    fields: &mut Vec<Vec<u8>>,
    field: &mut Vec<u8>,
    candles: &mut Vec<Candle>,
    is_header: &mut bool,
) -> Result<(), Box<dyn Error>> {
    // println!("{:?}", chunk);
    for byte in chunk {
        if *is_header {
            if *byte == 10 {
                *is_header = false;
            }
            continue;
        }

        //comma split
        if *byte == 44 {
            fields.push(field.clone());
            field.clear();
            continue;
        }
        // dollar currency sign just ignore and dont add
        if *byte == 36 || *byte == 13 {
            continue;
        }
        // newline split
        if *byte == 13 || *byte == 10 {
            fields.push(field.clone());
            field.clear();
            let field_0: String = String::from_utf8(fields[0].clone())?; // this field needs to be u64

            let field_1: f64 = String::from_utf8(fields[1].clone())?.parse()?;
            let field_2: f64 = String::from_utf8(fields[2].clone())?.parse()?; // this field needs to be u64
            let field_3: f64 = String::from_utf8(fields[3].clone())?.parse()?; // this field needs to be u64
            let field_4: f64 = String::from_utf8(fields[4].clone())?.parse()?; // this field needs to be u64
            let field_5: f64 = String::from_utf8(fields[5].clone())?.parse()?; // this field needs to be u64
            // dbg!(&field_5);

            let candle: Candle = Candle {
                date: field_0.clone(),
                close: field_1.clone(),
                volume: field_2.clone(),
                open: field_3.clone(),
                high: field_4.clone(),
                low: field_5.clone(),
            };
            candles.push(candle);
            fields.clear();
            continue;
        }
        field.push(*byte);
    }

    Ok(())
}
// // pub fn dataloader(data: &[u8]) -> Result<Vec<Candle>, Box<dyn Error>> {
//     let mut fields: Vec<Vec<u8>> = vec![];
//     let mut field: Vec<u8> = vec![];
//     let mut candle_array: Vec<Candle> = vec![];
//     //  let candle:Candle;
//     // maybe have a delimter at the comma

//     for byte in data {
//         if is_header_row {
//             if *byte == 10 {
//                 is_header_row = false;
//             }
//             continue;
//         }

//         //comma split
//         if *byte == 44 {
//             fields.push(field.clone());
//             field.clear();
//             continue;
//         }
//         // dollar currency sign
//         if *byte == 36 {
//             continue;
//         }
//         // newline split
//         if *byte == 13 || *byte == 10 {
//             fields.push(field.clone());
//             field.clear();
//             let field_0: String = String::from_utf8(fields[0].clone())?; // this field needs to be u64

//             let field_1: f64 = String::from_utf8(fields[1].clone())?.parse()?;
//             let field_2: f64 = String::from_utf8(fields[2].clone())?.parse()?; // this field needs to be u64
//             let field_3: f64 = String::from_utf8(fields[3].clone())?.parse()?; // this field needs to be u64
//             let field_4: f64 = String::from_utf8(fields[4].clone())?.parse()?; // this field needs to be u64
//             let field_5: f64 = String::from_utf8(fields[5].clone())?.parse()?; // this field needs to be u64
//             // dbg!(&field_5);

//             let candle: Candle = Candle {
//                 date: field_0.clone(),
//                 close: field_1.clone(),
//                 volume: field_2.clone(),
//                 open: field_3.clone(),
//                 high: field_4.clone(),
//                 low: field_5.clone(),
//             };
//             candle_array.push(candle);
//             continue;
//         }
//         field.push(*byte);
//     }

//     Ok(candle_array)
// }
