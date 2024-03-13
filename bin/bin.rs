use std::{path::Path};
use syue::csv;


fn main() {
      let file_path = Path::new("./bin/assets/data.csv");

      let r = csv::CsvReader::new(file_path);

      let x = r.csv_into_iter();

      dbg!(x);

}
