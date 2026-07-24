use crate::data::candle::Candle;
use std::error::Error;

pub fn parse_candles(
    fields: &mut Vec<Vec<u8>>,
    candles: &mut Vec<Candle>,
) -> Result<(), Box<dyn Error>> {
    let field_0:String= str::from_utf8(&fields[0])?.to_string(); // this field needs to be u64

    let field_1: f64 =str::from_utf8(&fields[1])?.parse()?;
    let field_2: f64 = str::from_utf8(&fields[2])?.parse()?; // this field needs to be u64
    let field_3: f64 = str::from_utf8(&fields[3])?.parse()?; // this field needs to be u64
    let field_4: f64 = str::from_utf8(&fields[4])?.parse()?; // this field needs to be u64
    let field_5: f64 = str::from_utf8(&fields[5])?.parse()?; // this field needs to be u64
    // dbg!(&field_5);

    let candle: Candle = Candle {
        date: field_0,
        close: field_1,
        volume: field_2,
        open: field_3,
        high: field_4,
        low: field_5,
    };
    candles.push(candle);

    Ok(())
}
