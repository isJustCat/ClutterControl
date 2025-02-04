use cluttercontrol_base::{models::*, storage::*, App};
use rocket::{
    get,
    serde::json::Json,
    Request, State,
};
use serde_json::{json, Value};

#[get("/everything")]
pub async fn r_get_everything(app: &State<App>) -> Json<Storage> {
    Json((*app.storage.read().await).clone())
}

#[get("/<identifier>/<field>/<value>")]
pub async fn r_query_storage(app: &State<App>, identifier: &str, field: &str, value: &str) -> Json<Value> {
    Json(json!(app.storage.write().await.query(identifier, field, value)))
}

#[put("/<identifier>", data="<object>")]
pub async fn r_create_object(app: &State<App>, identifier: &str, object: Json<Value>) {
    let mut storage = app.storage.write().await;
    match identifier {
        "locations" => storage.locations.push(serde_json::from_value((*object).clone()).expect("Oopsie woopsie! Seems like you fed me invalid data ><")),
        "creatures" => storage.creatures.push(serde_json::from_value((*object).clone()).expect("Oopsie woopsie! Seems like you fed me invalid data ><")),
        "items" => storage.items.push(serde_json::from_value((*object).clone()).expect("Oopsie woopsie! Seems like you fed me invalid data ><")),
        _ => storage.properties.push(serde_json::from_value((*object).clone()).expect("Oopsie woopsie! Seems like you fed me invalid data ><"))
    }
    storage.save();
}

#[delete("/<identifier>/<field>/<value>")]
pub async fn r_delete_object(app: &State<App>, identifier: &str, field: &str, value: &str) {
    let mut storage = app.storage.write().await;
    let objects  = storage.query(identifier, field, value);
    for k in objects.keys() {
        match identifier {
            "locations" => drop(storage.locations.remove(*k)),
            "creatures" => drop(storage.creatures.remove(*k)),
            "items" => drop(storage.items.remove(*k)),
            _ => drop(storage.properties.remove(*k))
        }
    }
    
}

