mod lib;
use std::{fs::File, io::Read};

use lib::dataloader;

fn main() {
    // let test_data = b"07/23/2026,$321.66,40840780,$321.73,$323.30,$319.35\n";
    let mut file_handle = File::open("test_data/test_data_file_1.csv").unwrap();
    let mut buffer = vec![];
    file_handle.read_to_end(&mut buffer).unwrap();
    
  let candle_array = dataloader(&buffer);

  println!("{:?}",candle_array);

}
