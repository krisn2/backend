use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: ObjectId,
    pub username: String,
    pub number : String,
    pub email: String, // adding other fliled after main code complete and test is succeded
}

