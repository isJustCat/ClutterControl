use serde::{Serialize, Deserialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path
};
use crate::models::*;

// i'm not sure if i am happy with this...
#[derive(Serialize, Deserialize, Debug)]
pub struct Storage {
    pub locations: Vec<Location>,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
    pub statuses: Vec<Status>,
    pub creatures: Vec<Creature>,
    pub custom_fields: Vec<CustomField>,
    pub items: Vec<Item>,
    pub item_quantities: Vec<ItemQuantity>,
    pub item_values: Vec<ItemValue>,
    pub item_locations: Vec<ItemLocation>,
    pub item_labels: Vec<ItemLabel>,
    pub item_statuses: Vec<ItemStatus>,
    pub item_expiries: Vec<ItemExpiry>,
    pub item_warranties: Vec<ItemWarranty>,
    pub item_owners: Vec<ItemOwner>,
    pub item_carriers: Vec<ItemCarrier>,
    pub item_categories: Vec<ItemCategory>,
    pub item_tags: Vec<ItemTag>,
    pub item_repairs: Vec<ItemRepair>,
    pub item_custom_fields: Vec<ItemCustomField>,
    pub item_tags: Vec<ItemTag>
}

// yea... someone once told us to "DRY" - not sure that's what they meant.
// need some input on this probably.
impl Storage {
    pub fn new() -> Self {
        Storage {
            locations: Vec::new(),
            categories: Vec::new(),
            tags: Vec::new(),
            statuses: Vec::new(),
            creatues: Vec::new(),
            custom_fields: Vec::new(),
            items: Vec::new(),
            item_quantities: Vec::new(),
            item_values: Vec::new(),
            item_locations: Vec::new(),
            item_labels: Vec::new(),
            item_statuses: Vec::new(),
            item_expiries: Vec::new(),
            item_warranties: Vec::new(),
            item_owners: Vec::new(),
            item_carriers: Vec::new(),
            item_categories: Vec::new(),
            item_tags: Vec::new(),
            item_repairs: Vec::new(),
            item_custom_fields: Vec::new(),
            item_tags: Vec::new()
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
        let storage: Storage = serde_json::from_str(&data);
        Ok(storage)
    }

pub fn query(&self, struct_name: &str, field: &str, value: &str) -> Vec<String> {
    let mut results = Vec::new();

    fn serch_in_vec<T: Serialize>(vec: &Vec<T>, field: &str, value: &str) -> Vec<String> {
        let mut found_items = Vec::new();

        for item in vec {
            // serialize object to json
            let json_value = serde_json::to_value(item).unwrap();

            // look for the specified field and compare it to the value
            if let some(field_value) = json_value.get(field) {
                if field_value == value {
                    // if it's a match, add it to the Vec of results
                    found_items.push(format!("{:?}", item))
                }
            }
        }

        found_items

    }
    match struct_name {
        "locations" => results.extend(search_in_vec(&self.locations, field, value)),
        "categories" => results.extend(search_in_vec(&self.categories, field, value)),
        "tags" => results.extend(search_in_vec(&self.tags, field, value)),
        "statuses" => results.extend(search_in_vec(&self.statuses, field, value)),
        "creatures" => results.extend(search_in_vec(&self.creatures, field, value)),
        "custom_fields" => results.extend(search_in_vec(&self.custom_fields, field, value)),
        "items" => results.extend(search_in_vec(&self.items, field, value)),
        "item_quantities" => results.extend(search_in_vec(&self.item_quantities, field, value)),
        "item_values" => results.extend(search_in_vec(&self.item_values, field, value)),
        "item_locations" => results.extend(search_in_vec(&self.item_locations, field, value)),
        "item_labels" => results.extend(search_in_vec(&self.item_labels, field, value)),
        "item_statuses" => results.extend(search_in_vec(&self.item_statuses, field, value)),
        "item_expiries" => results.extend(search_in_vec(&self.item_expiries, field, value)),
        "item_warranties" => results.extend(search_in_vec(&self.item_warranties, field, value)),
        "item_owners" => results.extend(search_in_vec(&self.item_owners, field, value)),
        "item_carriers" => results.extend(search_in_vec(&self.item_carriers, field, value)),
        "item_categories" => results.extend(search_in_vec(&self.item_categories, field, value)),
        "item_tags" => results.extend(search_in_vec(&self.item_tags, field, value)),
        "item_repairs" => results.extend(search_in_vec(&self.item_repairs, field, value)),
        "item_custom_fields" => results.extend(search_in_vec(&self.item_custom_fields, field, value)),
        _ => {} /
    }
    
}
}
