use std::fs::metadata;
use std::os::windows::fs::MetadataExt;
use std::time::UNIX_EPOCH;

pub fn exifextract(path: &String) {
    let path = path;

    match metadata(path) {
        Ok(meta) => {
            // File size
            let file_size = meta.len();
            println!("File size: {} bytes", file_size);

            // File permissions (Readonly)
            let permissions = meta.permissions();
            println!("Read-only: {}", permissions.readonly());

            // File attributes (Windows-specific)
            let attributes = meta.file_attributes();
            println!("File attributes: {}", attributes);

            if let Ok(access_time) = meta.accessed() {
                if let Ok(duration) = access_time.duration_since(UNIX_EPOCH) {
                    println!(
                        "Last accessed: {} seconds since UNIX EPOCH",
                        duration.as_secs()
                    );
                }
            }

            // Creation time
            if let Ok(creation_time) = meta.created() {
                if let Ok(duration) = creation_time.duration_since(UNIX_EPOCH) {
                    println!(
                        "Creation time: {} seconds since UNIX EPOCH",
                        duration.as_secs()
                    );
                }
            }

            if let Ok(modified_time) = meta.modified() {
                if let Ok(duration) = modified_time.duration_since(UNIX_EPOCH) {
                    println!(
                        "Last modified: {} seconds since UNIX EPOCH",
                        duration.as_secs()
                    );
                }
            }
        }
        Err(e) => {
            println!("Error retrieving metadata: {}", e);
        }
    }
}
