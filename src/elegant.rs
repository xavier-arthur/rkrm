use std::{
    collections::HashMap,
    path::Path
};

use sqlite::State;

use crate::hashmap;

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

    pub fn select<T>(&self, table: T, condition: T, columns: &[T]) -> HashMap<String, String> 
    where
        T: AsRef<str>
    {
        let mut hmap: HashMap<String, String> = hashmap![];

        let table     = table.as_ref();
        let condition = condition.as_ref();

        let cols = columns.into_iter()
            .fold(String::new(), |carry, v| {
                let separator = if carry.len() > 0 {
                    ","
                } else {
                    ""
                };

                format!("{carry}{separator} {}", v.as_ref().to_owned())
            });


        let sql = format!("SELECT {cols} FROM {table} WHERE {condition}");
        let mut stmt = match self.connection.prepare(sql) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("incorrect statament parsed at elegant\n\n{}", e.to_string());
                std::process::exit(1);
            }
        };

        while let Ok(State::Row) = stmt.next() {
            for item in columns {
                let item_string = item.as_ref().to_owned();

                hmap.insert(
                    item_string,
                    stmt.read::<String, _>(item.as_ref()).unwrap()
                );
            }
        }

        hmap
    }

    pub fn update(&mut self, table: &str, data: HashMap<&str, Option<&str>>, clause: &str) {

        todo!()
    }
}