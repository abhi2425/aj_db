#![allow(unused)]

use std::collections::*;
use std::env::*;
use std::fs::*;
use std::path::Path;

fn main() {
    let arguments = args();
    // println!("Entered arguments-:{arguments:?}");

    let mut valid_args = arguments.skip(1);

    //COMMAND
    let command = match valid_args.next() {
        Some(cmd) => cmd,
        None => {
            println!("cmd not found!");
            "".to_owned()
        }
    };

    //KEY
    let key = match valid_args.next() {
        Some(cmd) => cmd,
        None => {
            println!("key not found!");
            "".to_owned()
        }
    };

    // --------------------DATABASE INIT--------------------------//
    let mut database = Database::new().expect("Db creation failed");

    // --------------------GET--------------------------//
    if (&command == "get") {
        println!("Key-: {key}");
        let value = match database.find(&key) {
            Ok(value) => value,
            Err(err) => panic!("{err}"),
        };
        println!("Value-: {value}");
        return;
    }
    // --------------------SET--------------------------//
    if (&command == "set") {
        //VALUE
        let value = match valid_args.next() {
            Some(cmd) => cmd,
            None => {
                println!("value not found!");
                "".to_owned()
            }
        };
        if (!(key == "" && value == "")) {
            // database.insert(key.to_uppercase(), value.clone()); // just for fun
            database.insert(key, value);
            database.flush();
        }
        println!("Added Successfully!");
        return;
    }
    println!("Couldn't perform the operation")
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    // LOAD DATA IN HASHMAP AFTER READ
    fn new() -> Result<Self, std::io::Error> {
        let path_exist = Path::new("./data/ab.db").exists();

        if (!path_exist) {
            create_dir("./data");
            File::create("./data/ab.db");
        }

        let data = Self::read()?;
        let mut map = HashMap::new();
        for (key, value) in data {
            map.insert(key, value);
        }

        Ok(Self { map })
    }

    // READ DATABASE
    fn read() -> Result<Vec<(String, String)>, std::io::Error> {
        // let contents = match read_to_string("./data/ab.db") {
        //     Ok(c) => c,
        //     Err(e) => return Err(e),
        // };

        let contents = read_to_string("./data/ab.db")?;
        let mut data = vec![];
        // could have directly returned map instead of Vec and saved lot of memory but just wanted to play with return types
        for line in contents.lines() {
            let mut chunks = line.splitn(2, "\t");
            let key = chunks.next().expect("No Key!").to_owned();
            let value = chunks.next().expect("No Value!").to_owned();
            data.push((key, value));
        }
        Ok(data)
    }

    // INSERT INTO DATABASE
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    // WRITE ON DATABASE
    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            // let kv_pair = format!("{key}\t{value}\n");
            // contents.push_str(&kv_pair);
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        write("./data/ab.db", contents)
    }

    // FIND VALUE BY KEY NAME
    fn find(&self, key: &String) -> Result<String, std::io::Error> {
        let value = match self.map.get(key) {
            Some(value) => value,
            None => "",
        };

        Ok(value.to_owned())
    }
}
