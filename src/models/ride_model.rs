use serde::{Deserialize, Serialize};

use crate::models::driver_model::Driver;

#[derive(Deserialize, Serialize, Debug)]
pub struct RideRequest {
   pub rider_id: i32,
   pub pickup_location: String,
   pub destination: String,
    // adding other fliled after main code complete and test is succeded
}

#[derive(Deserialize, Serialize)]
pub struct RideResponse {
    pub ride_id :String,
    pub driver : Driver,
}

#[derive(Deserialize, Serialize)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}
