pub struct Candle{
    date:String,
    close:u64,
    volume:u64,
    open:u64,
    high:u64,
    low:u64,
}

// This will convert the csv format historical data in array of candles
pub fn dataloader(data:&[u8]){

 let mut field_data:Vec<Vec<u8>>;   
 let mut field: Vec<u8> = vec![];
 let candle_array:Vec<Candle>;
 let candle:Candle;
  // maybe have a delimter at the comma
let mut is_header_row:bool = true;

  for byte in data{
    field.push(*byte);
    if *byte == 44 {
        field_data.push(field.clone());
    }
  }
}