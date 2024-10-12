use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

use crate::models::ride_model::Location;

#[derive(Deserialize, Serialize, Debug)]
pub struct Driver {
    pub id: ObjectId,
    pub username: String,
    pub number : String,
    pub email: String, // adding other fliled after main code complete and test is succeded
    pub is_available: bool,
    pub current_location: Location,
}

