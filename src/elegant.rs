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

    fn parse_values(data: &mut HashMap<&str, Option<String>>) {
        for (_, v) in data.iter_mut() {
            if v.is_none() {
                *v = Some("NULL".to_string());
            } else {
                let formatted = format!("'{}'", v.clone().unwrap());
                *v = Some(formatted);
            }
        }
    }

    pub fn insert(&mut self, table: &str, mut data: HashMap<&str, Option<String>>) -> Result<(), sqlite::Error> {
        Self::parse_values(&mut data);

        let mut key_pair: Vec<(&str, Option<String>)> = vec![];

        for (k, v) in data.into_iter() {
            key_pair.push((k, v));
        }

        let keys: String = key_pair.iter()
            .map(|(k, _)| *k)
            .collect::<Vec<&str>>()
            .join(",");

        let values: String = key_pair.into_iter()
            .map(|(_, v)| v.unwrap())
            .collect::<Vec<String>>()
            .join(",");


        let sql = String::from(format!("INSERT INTO {table} ({keys}) VALUES ({values});"));

        self.connection.execute(sql)
    }

    pub fn update(&mut self, table: &str, data: HashMap<&str, Option<&str>>, clause: &str) {

        todo!()
    }
}