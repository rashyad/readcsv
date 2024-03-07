use std::fs;
use std::path::Path;
use serde_json::{Map, Value};


fn read_csv_into_json(file_path: &Path){


   let contents = fs::read_to_string(file_path)
   .expect("Should have been able to read the file");

   let cloned_contents = contents.clone();

   let rows: Vec<&str> = cloned_contents.lines().into_iter().collect();

   let row_count = cloned_contents.lines().into_iter().count();

   println!("{:?}", row_count);

   let mut vec = Vec::new();

   let header = rows[0];
   //let mut map = Map::new();

   for row in rows {
      //println!("{}", row)

      vec.push(row)
      
   }
   println!("{}", header)
   // // let collection = parts.collect::<Vec<&str>>();

   // // //map.insert("", Value::String(val));
   // // for a in collection.into_iter() {
   // //    println!("{:?}", a)
   // // }

   // let obj = Value::Object(map);

   //dbg!(vec);
}

fn read_csv_into_iter(file_path: &Path){
   
   let contents = fs::read_to_string(file_path)
   .expect("Should have been able to read the file");

   let cloned_contents = contents.clone();

   let rows: Vec<&str> = cloned_contents.lines().into_iter().collect();

   let row_count = cloned_contents.lines().into_iter().count();

   println!("{:?}", row_count);

   let mut vec = Vec::new();

   let header = rows[0];
   //let mut map = Map::new();

   for row in rows {
      //println!("{}", row);
      let split_row: Vec<&str> = row
      .split_inclusive(",")
      .into_iter()
      .collect();

      vec.push(split_row)
      
   }
   //println!("{}", header)
   // // let collection = parts.collect::<Vec<&str>>();

   // // //map.insert("", Value::String(val));
   // // for a in collection.into_iter() {
   // //    println!("{:?}", a)
   // // }

   // let obj = Value::Object(map);

   dbg!(vec);
}

fn main() {
   let file_path = Path::new("./src/cust100.csv");

   read_csv_into_iter(file_path)

}