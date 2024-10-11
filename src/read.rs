use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
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

    // Step 1: Locate the EXIF marker (0xFFE1)
    let mut buffer = [0; 2];
    file.seek(SeekFrom::Start(2))?; // Skip JPEG SOI marker
    loop {
        file.read_exact(&mut buffer)?;
        if &buffer == b"\xFF\xE1" {
            break; // Found EXIF marker
        }

        let segment_size = file.read_u16::<BigEndian>()?; // Read segment length
        file.seek(SeekFrom::Current((segment_size - 2) as i64))?; // Skip to the next segment
    }

    // Step 2: Read the EXIF header and check byte order
    let mut exif_header = [0; 6];
    file.read_exact(&mut exif_header)?;

    if &exif_header != b"Exif\0\0" {
        println!("EXIF header not found.");
        return Ok(());
    }

    // Step 3: Determine byte order (TIFF header)
    let byte_order = file.read_u16::<BigEndian>()?;
    let mut is_little_endian = false;
    if byte_order == 0x4949 {
        is_little_endian = true;
    } else if byte_order != 0x4D4D {
        println!("Unknown byte order.");
        return Ok(());
    }

    let mut offset_reader = if is_little_endian {
        file.read_u32::<LittleEndian>()?
    } else {
        file.read_u32::<BigEndian>()?
    };

    // Step 4: Move to the first Image File Directory (IFD) offset
    file.seek(SeekFrom::Start(offset_reader as u64))?;

    // Step 5: Read the number of tags in the IFD
    let num_tags = if is_little_endian {
        file.read_u16::<LittleEndian>()?
    } else {
        file.read_u16::<BigEndian>()?
    };

    println!("Number of EXIF tags: {}", num_tags);

    // Step 6: Parse each EXIF tag
    for _ in 0..num_tags {
        let tag_id = if is_little_endian {
            file.read_u16::<LittleEndian>()?
        } else {
            file.read_u16::<BigEndian>()?
        };

        let data_type = if is_little_endian {
            file.read_u16::<LittleEndian>()?
        } else {
            file.read_u16::<BigEndian>()?
        };

        let data_count = if is_little_endian {
            file.read_u32::<LittleEndian>()?
        } else {
            file.read_u32::<BigEndian>()?
        };

        let data_offset = if is_little_endian {
            file.read_u32::<LittleEndian>()?
        } else {
            file.read_u32::<BigEndian>()?
        };

        println!(
            "Tag ID: 0x{:04X}, Type: {}, Count: {}, Offset/Value: 0x{:08X}",
            tag_id, data_type, data_count, data_offset
        );
    }

    Ok(())
}
