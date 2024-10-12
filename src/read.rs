use chrono::DateTime;
use jpeg_decoder::{Decoder, ImageInfo};
use prettytable::{Cell, Row, Table};
use std::fs::{metadata, File};
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};
use colored::*;

pub fn exifextract(path: &str) {
    // Open the JPEG file
    let file = File::open(path).expect("Failed to open the image file");
    let reader = BufReader::new(file);

    // Decode the JPEG image
    let mut decoder = Decoder::new(reader);
    let _ = decoder.decode().expect("Failed to decode the image");

    // Access the metadata (includes EXIF)
    if let Some(info) = decoder.info() {
        print_image_info_in_table(info, path);
    } else {
        println!("No EXIF metadata found.");
    }
}

fn print_image_info_in_table(info: ImageInfo, path: &str) {
    // Create a table to display the image information
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new(&"Field".green().to_string()), Cell::new(&"Value")]));

    // Add rows with image information to the table
    table.add_row(Row::new(vec![
        Cell::new(&"Dimensions".green().to_string()),
        Cell::new(&format!("{} x {}", info.width, info.height)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new(&"Color Type".green().to_string()),
        Cell::new(&format!("{:?}", info.pixel_format)),
    ]));

    if let Ok(metadata) = metadata(path) {
        if let Ok(modified) = metadata.modified() {
            let modified_time = system_time_to_date_time(modified);
            table.add_row(Row::new(vec![
                Cell::new(&"Last Modified".green().to_string()),
                Cell::new(&modified_time),
            ]));
        }

        if let Ok(created) = metadata.created() {
            let created_time = system_time_to_date_time(created);
            table.add_row(Row::new(vec![
                Cell::new(&"Creation Time".green().to_string()),
                Cell::new(&created_time),
            ]));
        }

        let file_size = metadata.len();
        table.add_row(Row::new(vec![
            Cell::new(&"Size".green().to_string()),
            Cell::new(&format!("{} bytes", file_size)),
        ]));
    }
    // Display the table
    table.printstd();
}

// Helper function to convert SystemTime to a human-readable date-time string
fn system_time_to_date_time(system_time: SystemTime) -> String {
    let datetime = system_time
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let naive = DateTime::from_timestamp(datetime as i64, 0);
    naive
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "N/A".to_string())
}
