use std::{collections::HashMap, str::FromStr};

use cluttercontrol_base::{models::*, Storage, App, FsExt};
use rocket::{
    get, http::Status, serde::json::{from_str, Json}, Request, State
};
use serde_json::{json, Value};
use uuid::Uuid;

#[get("/everything")]
pub async fn r_get_everything(app: &State<App>) -> Json<Storage> {
    Json((*app.storage.read().await).clone())
}

#[put("/", data="<objects>")]
pub async fn r_create_objects(app: &State<App>, mut objects: Json<Vec<Value>>) -> Json<HashMap<Uuid, Value>>{
    let mut storage = app.storage.write().await;
    let mut created_objects = HashMap::new();
    for object in objects.iter() {
        let entity: Entity = serde_json::from_str(&object.to_string()).expect("Failed to deserialize object");
        let uuid = Uuid::new_v4();
        storage.insert(uuid.clone(), entity);
        created_objects.insert(uuid, object.clone());
    }

    storage.save();
    Json(created_objects)
}


/// Delete an Entity from the Storage
/// i
/// Arguments
/// `identifier` - UUID of the Entity to be deleted

#[delete("/<identifier>")]
pub async fn r_delete_object(app: &State<App>, identifier: String) -> Result<Json<serde_json::Value>, Status> {
    let mut storage = app.storage.write().await;

    match Uuid::from_str(&identifier) {
        Ok(uuid) => {
            if storage.remove(&uuid).is_some() {
                storage.save();
                Ok(Json(json!({"message": "Object deleted"})))
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => {
            Err(Status::BadRequest)
        }
    }
}

// TODO: Implement update endpoint lol

