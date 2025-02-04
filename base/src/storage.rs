

#![allow(unused)]

use serde::{Serialize, Deserialize};
use std::{
    fmt::Debug, fs::File, io::Read
};
use crate::models::*;




// I'm not sure if I am happy with this...
#[derive(Serialize, Deserialize, Debug)]
pub struct Storage {
    pub locations: Vec<Location>,
    pub creatures: Vec<Creature>,
    pub items: Vec<Item>,
    pub properties: Vec<Property>,
    pub changes: Vec<Change>,
    pub actions: Vec<Action>,

}

impl Storage {
    pub fn new() -> Self {
        Storage {
            locations: Vec::new(),
            creatures: Vec::new(),
            items: Vec::new(),
            properties: Vec::new(),
            changes: Vec::new(),
            actions: Vec::new()
        }
    }


    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }

    pub fn load(path: &str) -> std::io::Result<Storage> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let storage: Storage = serde_json::from_str(&data).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        })?;
        Ok(storage)
    }

    /// Query some struct the storage for a given key-value pair.
    /// 
    /// Arguments
    /// * `struct_name` 
    /// *  `field`
    /// * `value`
    /// 
    /// Example
    /// ``` rust
    /// storage::query("locations", "name", "kitchen shelf");
    /// ```
    pub fn query(&self, identifier: &str, field: &str, value: &str) -> Vec<String> {
        let mut results = Vec::new();

        fn search_in_vec<T: Serialize + Debug>(vec: &Vec<T>, node: Option<&str>, field: &str, value: &str) -> Vec<String> {
            let mut found_items = Vec::new();

            for item in vec {
                // serialize object to json
                let json_value = match serde_json::to_value(item) {
                    Ok(value) => value,
                    Err(_) => continue, // skip this item if serialization fails
                };

                // look for the specified field and compare it to the value
                if let Some(field_value) = json_value.get(field) {
                    if field_value == value {
                        // if it's a match, add it to the Vec of results
                        found_items.push(format!("{:?}", item))
                    }
                }
            }

            found_items
        }

        match identifier {
            "locations" => results.extend(search_in_vec(&self.locations, None, field, value)),
            "creatures" => results.extend(search_in_vec(&self.creatures, None, field, value)),
            "items" => results.extend(search_in_vec(&self.items, None,  field,  value)),
            _ => results.extend(search_in_vec(&self.properties, Some(identifier), field, value))
        }

        results
    }
}
