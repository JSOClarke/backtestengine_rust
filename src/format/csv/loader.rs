use std::fs::File;
use std::io::Read;
use crate::data::candle::Candle;
use crate::format::csv::parser::parse_csv;
use std::error::Error;



pub fn handle_csv(file: &mut File) -> Result<Vec<Candle>, Box<dyn Error>> {
    let mut buffer = [0u8; 16];
    let mut fields: Vec<Vec<u8>> = vec![];
    let mut field: Vec<u8> = vec![];
    let mut candles: Vec<Candle> = vec![];
    let mut is_header: bool = true;
    let expected_h_count: usize = 5;

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
            &expected_h_count,
        )?;
    }
    Ok(candles)
}