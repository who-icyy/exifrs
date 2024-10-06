use std::path;

use clap::{Arg, Command};
mod read;
fn main() {
    let matches = Command::new("<File location>")
        .version("1.0.0")
        .author("@Who-icyy")
        .about("Exiftool clone but in RUST Programming.")
        .arg(
            Arg::new("path")
                .help("<Target file path>")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("write")
                .short('w')
                .long("write")
                .help("<Write Metadata to file.>")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let file_path: &String = matches.get_one::<String>("path").unwrap();

}
