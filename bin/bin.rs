use std::path::Path;
use syue::csv;

fn main(){
   let file_path = Path::new("./bin/assets/data.csv");

   print!("{:?}", file_path);

   let new_reader = csv::CsvReader::new(file_path);

   let data = new_reader.csv_into_iter();

   dbg!(data);

}