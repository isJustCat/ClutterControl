use std::{collections::HashMap, str::FromStr};

use cluttercontrol_base::{models::*, App, Storage};
use rocket::{
    get, http::Status, response::status, serde::json::{self, Json}, State
};
use serde_json::{json, Value};
use uuid::Uuid;
use anyhow::{Error, Result};

use crate::ApiResponse;

// Create a new entity
#[post("/", data = "<objects>")]
pub async fn r_create(app: &State<App>, objects: Json<Vec<Value>>) -> Result<ApiResponse, ApiResponse> {
    let mut storage = app.storage.write().await;
    let mut created_objects = HashMap::new();

    for object in objects.iter() {
        match serde_json::from_value(object.clone()) {
            Ok(entity) => {
                let uuid = Uuid::new_v4();
                storage.0.insert(uuid, entity);
                created_objects.insert(uuid, object.clone());
            }
            Err(e) => {
                return Err(ApiResponse {
                    json: json!{format!("Failed to deserialize object {}", e)},
                    status: Status::ImATeapot,
                });
            }
        }
    }

    if let Err(e) = Storage::save(&storage) {
        eprintln!("Failed to save storage: {}", e);
        return Err(ApiResponse {
            json: json!("Failed to save storage"),
            status: Status::InternalServerError,
        });
    }

    Ok(ApiResponse {
        json: json!(created_objects),
        status: Status::Created,
    })
}

/// Get all entities of a given type or one matching by uuid
#[get("/?<id>&<e_type>&<field>")]
pub async fn r_get(app: &State<App>, id: Option<String>, e_type: Option<String>, field: Option<String>) -> Result<ApiResponse, ApiResponse> {
    let storage = app.storage.read().await;
    let mut result = HashMap::new();

    if id.is_none() && e_type.is_none() {
        return Ok(ApiResponse {
            json: json!(storage.0.clone()),
            status: Status::Ok,
        });
    }

    if let Some(id) = id {
        match Uuid::from_str(&id) {
            Ok(uuid) => {
                if let Some(entity) = storage.0.get(&uuid) {
                    result.insert(uuid, entity.clone());
                } else {
                    return Err(ApiResponse {
                        json: json!(format!("Failed to find item with given UUID {}", uuid)),
                        status: Status::NotFound,
                    });
                }
            }
            Err(_) => {
                return Err(ApiResponse {
                    json: json!("Failed to serialize UUID"),
                    status: Status::BadRequest,
                });
            }
        }
    }

    if let Some(e_type) = e_type {
        Storage::match_entity_type(&storage, &e_type, |uuid, entity| {
           result.insert(*uuid, entity.clone());
        });
        
    }

    if result.is_empty() {
        return Err(ApiResponse {
            json: json!("No items found"),
            status: Status::NotFound,
        });
    }

    Ok(ApiResponse {
        json: json!(result),
        status: Status::Ok,
    })
}

/// Update any given number of entities
#[put("/", data = "<objects>")]
pub async fn r_update(app: &State<App>, objects: Json<HashMap<String, Value>>) -> Result<ApiResponse, ApiResponse> {
    let mut storage = app.storage.write().await;
    let mut updated_objects = HashMap::new();

    for (uuid, object) in objects.iter() {
    
        let uuid = match Uuid::from_str(uuid) {
            Ok(uuid) => uuid,
            Err(_) => {
                return Err(ApiResponse {
                    json: json!("Failed to serialize UUID"),
                    status: Status::BadRequest,
                });
            }
        };

        if let Some(existing_entity) = storage.0.get_mut(&uuid) {
            match serde_json::from_value(object.clone()) {
                Ok(updated_entity) => {
                    *existing_entity = updated_entity;
                    updated_objects.insert(uuid, existing_entity.clone());
                }
                Err(_) => {
                    return Err(ApiResponse {
                        json: json!("Failed to deserialize object"),
                        status: Status::BadRequest,
                    });
                }
            }
        } else {
            return Err(ApiResponse {
                json: json!(format!("Failed to find item with given UUID {}", uuid)),
                status: Status::NotFound,
            });
        }
    }

    if let Err(e) = Storage::save(&storage) {
        eprintln!("Failed to save storage: {}", e);
        return Err(ApiResponse {
            json: json!("Failed to save storage"),
            status: Status::InternalServerError,
        });
    }

    Ok(ApiResponse {
        json: json!(updated_objects),
        status: Status::Ok,
    })
}

/// Delete any given number of entities
#[delete("/", data = "<identifiers>")]
pub async fn r_delete(app: &State<App>, identifiers: Json<Vec<Value>>) -> Result<ApiResponse, ApiResponse> {
    let mut storage = app.storage.write().await;
    let ids: Vec<Uuid> = identifiers
        .iter()
        .filter_map(|value| value.as_str().and_then(|s| Uuid::from_str(s).ok()))
        .collect();

    for id in ids {
        storage.0.remove(&id);
    }

    if let Err(e) = Storage::save(&storage) {
        eprintln!("Failed to save storage: {}", e);
        return Err(ApiResponse {
            json: json!("Failed to save storage"),
            status: Status::InternalServerError,
        });
    }

    Ok(ApiResponse {
        json: json!(""),
        status: Status::Ok,
    })
}