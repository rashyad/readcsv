use regex::Regex;
use serde_json::{Map, Value};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::String;

pub struct CsvReader {
   file_path: String,
}

impl CsvReader {
   pub fn new(file_path: &Path) -> Self {
      CsvReader {
         file_path: String::from(file_path.to_str().unwrap()),
      }
   }

   pub fn csv_into_json(&self) -> Vec<Value> {
        
      let f = File::open(self.file_path.as_str()).expect("Expecting a file");

      let reader = BufReader::new(f);

      let mut lines = reader.lines();

      let first_vec = lines.nth(0).unwrap();

      let content_vec: Vec<_> = lines.collect();

      let header: Vec<String> = first_vec.unwrap()
         .split(',')
         .map(|x| x.to_lowercase().replace(' ', "").to_string())
         .collect();

      let split_rows = content_vec.into_iter().map(|line| {
         let mut row = line.expect("Error reading line");
         
         let pattern = format!(r"{}", "\"(.*)\"");

         let re = Regex::new(pattern.as_str()).unwrap();
         
         if re.is_match(&row) {
            let x = re.find(&row).unwrap();

            let replaced_string = x.as_str().replace(',', "");

            row.replace_range(x.range(), &replaced_string);
         }

         let split_row: Vec<String> =
            row.split(',').map(|y| y.to_string()).collect();

         split_row
      });

      let result_vec: Vec<Value> = split_rows
         .map(|row| {
               let map: Map<String, Value> = Map::new();
               let mut obj = Value::Object(map);

               let mut i = 0;
               while i < header.len() {
                  let h = header[i].clone();
                  let val = row[i].clone();

                  if let Value::Object(ref mut map) = obj {
                     map.insert(h, Value::String(val));
                  }

                  i += 1;
               }
               obj
         })
         .collect();

      result_vec
    }

   pub fn csv_into_iter(&self) -> Vec<Vec<String>> {

      let f = File::open(self.file_path.as_str()).expect("Expecting a file");

      let reader = BufReader::new(f);

      let lines  = reader.lines();

      let result: Vec<Vec<String>> = lines.map(|line| {
           
         let mut row = line.expect("Error reading line");

         let pattern = format!(r"{}", "\"(.*)\"");

         let re = Regex::new(pattern.as_str()).unwrap();
         
         if re.is_match(&row) {
            let x = re.find(&row).unwrap();

            let replaced_string = x.as_str().replace(',', "");

            row.replace_range(x.range(), &replaced_string);
         }

         let split_line: Vec<String> =
            row.split(',').map(|y| y.to_string()).collect();

         split_line

      }).collect();

      result
   }

}
