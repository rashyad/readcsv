use std::path::Path;
use serde_json::{Map, Value};
use std::{ fs };
use std::string::String;

pub(crate) struct Reader {
    file_path: String
}
 
impl Reader {

    pub fn new(file_path: &Path) -> Self {
        Reader {
            file_path: String::from(file_path.to_str().unwrap()),
        }
    }

    pub(crate) fn xlsx_into_json(&self) -> Vec<Value>{
       let contents = fs::read_to_string(self.file_path.as_str())
       .unwrap();
    
       let content_cloned = contents.clone();
    
       let content_iter = content_cloned.lines().into_iter();
    
       let content_vec: Vec<&str> = content_cloned.lines().into_iter().collect();
    
       let header: Vec<String> = content_vec[0]
       .split(",")
       .into_iter()
       .map(|x| {
          let y = x
          .to_lowercase()
          .replace(" ", "")
          .to_string();
          y
       }).collect();
    
       //let mut v= Vec::new();   
    
       let split_rows = content_iter
       .map(|row| { 
          let split_row: Vec<String> = row
          .split(",")
          .into_iter()
          .map(|y| y.to_string())
          .collect();
    
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
       }).collect();
    
       result_vec
    }
 
    pub(crate) fn xlsx_into_iter(&self) -> Vec<Vec<String>>  {
    
       let contents = fs::read_to_string(self.file_path.as_str())
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
    
    
}
 