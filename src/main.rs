use clap::{Arg, Command};
mod extract;
fn main() {
    let matches = Command::new("<File location>")
    .version("1.0.0")
    .author("@Who-icyy")
    .about("Exiftool clone but in RUST Programming.")
    .arg(
        Arg::new("path")
        .help("<Target file path>")
        .required(true)
        .index(1)
    ).get_matches();

    let input: &String = matches.get_one::<String>("path").unwrap();

    extract::exifextract(input);
}
