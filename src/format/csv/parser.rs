use std::error::Error;

use crate::data::{candle::Candle, parser::parse_candles};

pub fn parse_csv(
    chunk: &[u8],
    fields: &mut Vec<Vec<u8>>,
    field: &mut Vec<u8>,
    candles: &mut Vec<Candle>,
    is_header: &mut bool,
    expected_h_count: &usize,
) -> Result<(), Box<dyn Error>> {
    // println!("{:?}", chunk);
    for byte in chunk {
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
        if *byte == 10 {
            fields.push(field.clone());
            field.clear();

            if *is_header {
                *is_header = false;
                if fields.len() - 1 != *expected_h_count {
                    return Err("incorrect number".into());
                } else {
                    fields.clear();
                    continue;
                }
            }

            parse_candles(fields, candles)?;
            fields.clear();
            continue;
        }
        field.push(*byte);
    }

    Ok(())
}