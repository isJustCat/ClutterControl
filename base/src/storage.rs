use serde::{Serialize, Deserialize};
use serde_json::{value::Index, Value};
use std::{
    collections::HashMap, fmt::Debug, fs::{File, OpenOptions}, io::Read, path::Path
};
use anyhow::{Error, Result};
use crate::models::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
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


    pub fn save(&self) -> Result<()> {
        let file = OpenOptions::new().read(true).write(true).create(true).open("storage.json")?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let mut file = File::open(Path::new("storage.json"))?;
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
    pub fn query(&self, identifier: &str, field: &str, value: &str) -> HashMap<usize, String> {
        let mut results = HashMap::new();

        fn search_in_vec<T: Serialize + for<'a> Deserialize<'a> + Debug>(vec: &Vec<T>, node: Option<&str>, field: &str, value: &str) -> HashMap<usize, String> {
            let mut found_items = HashMap::new();

            for (index, item) in vec.iter().enumerate() {
                // serialize object to json
                let json_value = match serde_json::to_value(item) {
                    Ok(value) => value,
                    Err(_) => continue, // skip this item if serialization fails
                };

                // look for the specified field and compare it to the value
                if let Some(field_value) = json_value.get(field) {
                    if field_value == value {
                        // if it's a match, add it to the Vec of results
                        found_items.insert(index, String::from(value));
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
        };

        results
    }
}
