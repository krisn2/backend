use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub number : String,
    pub email: String, // adding other fliled after main code complete and test is succeded
}

