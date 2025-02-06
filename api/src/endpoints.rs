use std::str::FromStr;

use cluttercontrol_base::{models::*, Storage, App, FsExt};
use rocket::{
    get,
    serde::json::{from_str, Json},
    Request, State,
};
use serde_json::{json, Value};
use uuid::Uuid;

#[get("/everything")]
pub async fn r_get_everything(app: &State<App>) -> Json<Storage> {
    Json((*app.storage.read().await).clone())
}

#[put("/<entity_type>", data="<object>")]
pub async fn r_create_object(app: &State<App>, entity_type: &str, mut object: Json<Value>) {
    let mut storage = app.storage.write().await;
    let mut entity: Entity;
    match entity_type {
        "creature" => {
            let entity: Creature = serde_json::from_str(&object.to_string()).unwrap();
        },
        "item" => {
            let entity: Item = serde_json::from_str(&object.to_string()).unwrap();
        },
        "location" => {
            let entity: Location = serde_json::from_str(&object.to_string()).unwrap();
        },
        "change" => {
            let entity: Change = serde_json::from_str(&object.to_string()).unwrap();
        },
        "condition" => {
            let entity: Condition = serde_json::from_str(&object.to_string()).unwrap();
        },
        "tag" => {
            let entity: Tag = serde_json::from_str(&object.to_string()).unwrap();
        }
        _ => {}
    }
    
}


/// Delete an Entity from the Storage
/// 
/// Arguments
/// `identifier` - UUID of the Entity to be deleted
#[delete("/<identifier>")]
pub async fn r_delete_object(app: &State<App>, identifier: String) {
    let mut storage = app.storage.write().await;
    let uuid = Uuid::from_str(&identifier).expect("Failed to parse UUID");
    storage.remove(&uuid); // remove Entity with the specified uuid 
    storage.save(); // write storage to disk
    
}

