use jpeg_decoder::Decoder;
use std::fs::File;
use std::io::BufReader;

pub fn exifextract(path: &String) {
    let file = File::open(path).expect("Failed to open the image file");
    let reader = BufReader::new(file);

    // Decode the JPEG image
    let mut decoder = Decoder::new(reader);
    let _ = decoder.decode().expect("Failed to decode the image");

    // Access the metadata (includes EXIF)
    if let Some(exif_data) = decoder.info() {
        println!("EXIF Metadata: {:?}", exif_data);
    } else {
        println!("No EXIF metadata found.");
    }
}
