use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use futures::StreamExt;
use mongodb::bson::oid::ObjectId;
use crate::models::ride_model::{RideRequest, Trip};
use crate::config::AppState;


pub async fn handle_ride_request(trip_req:web::Json<RideRequest>, data: web::Data<AppState>) -> impl Responder {
   let pickup_location = trip_req.pickup_location;
   let redis_conn = match data.redis_client.get_async_connection().await {
    Ok(conn) => conn,
    Err(e) => {
        return HttpResponse::InternalServerError().json(e.to_string());
    }
   };

   // find nearby drivers within 1 km radius
   let nearby_driver_ids: Vec<String> = match redis_conn
    .georadius("drivers", pickup_location.lng, pickup_location.lat, 1000.0, "m")
    .await {
        Ok(ids) => ids,
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    };
    if nearby_driver_ids.is_empty() {
        return HttpResponse::Ok().json("No drivers available nearby");
    }

    let driver_ids: Vec<ObjectId> = nearby_driver_ids.iter()
        .filter_map(|id| ObjectId::with_string(id).ok())
        .collect();

    //Filter drivers that are available from the database
    let filter = mongodb::bson::doc! {"_id": {"$in":driver_ids}, "is_available":true};
    let mut cursor = match data.driver_collection.find(filter).await {
        Ok(cursor) => cursor,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to fetch drivers from database")
    };

    let mut available_drivers = Vec::new();
    while let Some(driver) = cursor.next().await.unwrap() {
        available_drivers.push(driver);
    }

    HttpResponse::Ok().json(available_drivers)
}





pub async fn select_driver(driver_id: web::Path<ObjectId>,trip_req: web::Json<RideRequest>, data: web::Data<AppState>) -> impl Responder {
   
   //Fetch the selected driver from the database

   let driver = match data.driver_collection
   .find_one(mongodb::bson::doc! {"_id": driver_id.clone(), "is_available":true})
   .await {
    Some(driver) => driver,
    None => return HttpResponse::NotFound().json("Driver not found or not available")
   };

   //assign the driver to the trip
   let trip = Trip {
    id: ObjectId::new(),
    rider_id: trip_req.rider_id.clone(),
    driver_id: Some(driver_id.clone()),
    pickup: trip_req.pickup_location.clone(),
    destination: trip_req.destination_location.clone(),
    status: "requested".to_string(),
    start_time: Some(Utc::now()),
    end_time: None,
   };

   //store the trip in the database
   match data.trip_collection.insert_one(trip).await {
    Ok(_) => HttpResponse::Ok().json("Trip request successful"),
    Err(e) => HttpResponse::InternalServerError().json(e.to_string())
   }
}
