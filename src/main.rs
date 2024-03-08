use std::{error::Error, fs};
use std::path::Path;
use serde_json::{Map, Value};
use regex::Regex;

fn read_csv_into_json(file_path: &Path){


   let contents = fs::read_to_string(file_path)
   .expect("Should have been able to read the file");


}


fn read_csv_into_iter(file_path: &Path) -> Vec<Vec<String>>  {
   
   let contents = fs::read_to_string(file_path)
   .unwrap();

   let content_iter = contents.lines().into_iter();

   let result_vec: Vec<Vec<String>> = content_iter.map(|row| { 
      
      // TODO - implement REGEX
      let split_line: Vec<String> = row
      .split(",")
      .into_iter()
      .map(|y| y.to_string())
      .collect();

      split_line 
   }).collect();

   result_vec
 
}

fn main(){
   let file_path = Path::new("./src/cust100.csv");

   let x = read_csv_into_iter(file_path);

   dbg!(x);

}