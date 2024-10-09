// use actix_web::{};
use crate::models::ride_model::{RideRequest, RideResponse};
use crate::models::user_model::User;


    pub async fn create_ride(request: RideRequest, user: User) {
        // finding the driver in nearest proximity to the rider's location
        // Get the location of the rider from the request (lat, lng) and call the haversin distance function
        // and if the distance is less than a certain threshold, send a notification to the driver
        // else return ride unavailable
        // return a response with the driver's details
        
    }