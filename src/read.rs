use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

pub fn exifextract(path: &String) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = [0; 2];

    file.read_exact(&mut buffer)?;

    if &buffer == b"\xFF\xD8" {
        println!("This is a JPEG file.");
    } else {
        println!("Not a JPEG file.");
        return Ok(());
    }

    file.seek(SeekFrom::Start(2))?;
    let mut marker_buffer = [0; 4];

    while file.read_exact(&mut marker_buffer).is_ok() {
        if &marker_buffer[0..2] == b"\xFF\xE1" {
            println!("Found EXIF marker.");
            break;
        }

        let segment_length = u16::from_be_bytes([marker_buffer[2], marker_buffer[3]]);
        file.seek(SeekFrom::Current(i64::from(segment_length) - 2))?;
    }

    Ok(())
}
