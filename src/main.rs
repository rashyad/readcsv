use std::{ fs };
use std::path::Path;
use serde_json::{Map, Value};
mod csv;


fn main(){
   let file_path = Path::new("./src/cust100.csv");

   let x = csv::Reader::read_csv_into_iter(file_path);

   dbg!(x);

}