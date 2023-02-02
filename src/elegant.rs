use std::{
    collections::HashMap,
    path::Path
};

pub struct Elegant {
    connection: sqlite::Connection
}

impl Elegant {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        let con = sqlite::open(path.as_ref());

        match con {
            Ok(v) => { 
                Self {
                    connection: v
                }
            }, 

            Err(e) => panic!("couldn't create or stabilish connection with the storage")
        }
    }

    fn parse_values(data: &mut HashMap<&str, Option<&str>>) {
        for (k, v) in data.iter_mut() {
            if v.is_none() {
                *v = Some("NULL");
            } else {
                let value = format!("'{}'", v.unwrap());

                *v = Some(&value.to_string());
            }
        }
    }

    pub fn insert(&mut self, table: &str, mut data: HashMap<&str, Option<&str>>) {
        let sql = String::from(format!("INSERT INTO {} ", table));
        Self::parse_values(&mut data);

        println!("{data:#?}");
    }

    pub fn update(&mut self, table: &str, data: HashMap<&str, Option<&str>>, clause: &str) {

    }
}