

#[cfg(test)]
#[test]

fn test_detect_magic_number() {
    let test_file_16_png: [u8; 16] = [
        0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00, 0x00,
        0x01,
    ];
    let result: FileType = detect_magic_number(&test_file_16_png);
    assert_eq!(result, FileType::Png);
    assert_ne!(result, FileType::Csv);
}

pub fn test() {
    println!("Bitch")
}

pub fn detect_magic_number(sixteenbytes: &[u8]) -> FileType {
    // let mut file_type = String::new();
    match sixteenbytes {
        [0xFF, 0xD8, 0xFF, 0xE0, ..] => {
            return FileType::Png;
        }

        [0xf1, 0x00, 0x40, 0xbb, ..] => {
            return FileType::Jpeg;
        }
        _ => {
            return FileType::Unknown;
        }
    }
}
