use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

use crate::models::driver_model::Driver;

#[derive(Deserialize, Serialize, Debug)]
pub struct RideRequest {
   pub rider_id: i32,
   pub pickup_location: Location,
   pub destination: Location,
    // adding other fliled after main code complete and test is succeded
}

// #[derive(Deserialize, Serialize)]
// pub struct RideResponse {
//     pub ride_id :String,
//     pub driver : Driver,
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

pub enum Status {
    Requested = "requested",
    Assigned = "assigned",
    Started = "started",
    Completed = "completed",
    Cancelled = "cancelled",
}

#[derive(Serialize, Deserialize)]
pub struct Trip {
    pub id: ObjectId,
    pub rider_id: ObjectId,
    pub driver_id: Option<ObjectId>,
    pub pickup: Location,
    pub destination: Location,
    pub status: Status,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}