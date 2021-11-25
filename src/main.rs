use std::fmt::Error;
use std::{env::Args, iter::Skip};
use std::collections::HashMap;

fn main() {
   let mut arguments: Skip<Args> = std::env::args().skip(1);
   let key: String = arguments.next().unwrap();
   let value: String = arguments.next().expect("Value is not there");
   println!("The key is '{}' and the value is'{}'", key, value);

   let contents:String= format!("{}\t{}\t\n", key, value);
   std::fs::write("kv.db", contents).unwrap();

   let database: Database = Database::new();
}

struct Database{
    map: HashMap<String, String>,
}

impl Database{
    fn new() -> Result<Database, std::io::Error>{
        //read the kv.db file(Binding)
       let contents: String = match std::fs::read_to_string("kv.db"){
            Ok(c) =>c,
            Err(error) =>{
                return Result::Err(error);
            }
        };
        //parse the string
        //populate the file

        Database{
            map: HashMap::new(),
        }
    }
}

