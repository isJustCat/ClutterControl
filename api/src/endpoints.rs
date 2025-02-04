use cluttercontrol_base::{models::*, storage::*, App};
use rocket::{
    get,
    serde::json::Json,
    Request, State,
};
use serde_json::{json, Value};

#[get("/everything")]
pub fn r_get_everything(app: &State<App>) -> Json<Storage> {
    Json((*app.storage.read().unwrap()).clone())
}

#[get("/<identifier>/<field>/<value>")]
pub fn r_query_storage(app: &State<App>, identifier: &str, field: &str, value: &str) -> Json<Value> {
    Json(json!(app.storage.write().unwrap().query(identifier, field, value)))
}

#[put("/<identifier>", data="<object>")]
pub fn r_create_object(app: &State<App>, identifier: &str, object: Json<Value>) {
    let mut storage = app.storage.write().unwrap();
    match identifier {
        "locations" => storage.locations.push(serde_json::from_value((*object).clone()).expect("Oopsie woopsie! Seems like you fed me invalid data ><")),
        "creatures" => storage.creatures.push(serde_json::from_value((*object).clone()).expect("Oopsie woopsie! Seems like you fed me invalid data ><")),
        "items" => storage.items.push(serde_json::from_value((*object).clone()).expect("Oopsie woopsie! Seems like you fed me invalid data ><")),
        "changes" => storage.changes.push(serde_json::from_value((*object).clone()).expect("Oopsie woopsie! Seems like you fed me invalid data ><")), // actions only occur within changes so we do not need a create endpoint for actions.
        _ => storage.properties.push(serde_json::from_value((*object).clone()).expect("Oopsie woopsie! Seems like you fed me invalid data ><"))
    }
    storage.save();
}


