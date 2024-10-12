use polars::prelude::*;

pub fn Exif() -> PolarsResult<()> {
    // Create some sample data
    let names = Series::new("Name", &["Alice", "Bob", "Charlie"]);
    let ages = Series::new("Age", &[25, 30, 22]);
    let cities = Series::new("City", &["New York", "London", "Paris"]);

    // Create a DataFrame from the series
    let df = DataFrame::new(vec![names, ages, cities])?;

    // Print the DataFrame
    println!("{:?}", df);

    Ok(())
}
