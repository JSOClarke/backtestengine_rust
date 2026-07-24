use std::error::Error;

use backtestengine::{file_loader, utilities::magic_number::FileType};

fn main() -> Result<(), Box<dyn Error>> {
    let ft = FileType::Csv;
    file_loader(String::from("test_data/test_data_file_1.csv"), ft)?;
    Ok(())
}
