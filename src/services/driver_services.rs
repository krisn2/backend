use actix_web::{web, HttpResponse, Responder};

use crate::models::driver_model::Driver;
use crate::models::ride_model::Location;
use crate::config::AppState;

pub async fn update_driver_location(driver: Driver, location: Location, data: web::Data<AppState>) ->impl Responder {
    let mut redis_conn = data.redis_client.get_async_connection().await.unwrap();
    // redis_conn.set_ex(format!("driver:{}", driver.id), location, 3600).await.unwrap();
    // Ok(())
    let result: Result<(), _> = redis_conn
        .geoadd(
            "drivers",
            location.lng,
            location.lat,
            driver.id.to_string(),
        )
        .await;
        // .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    match result {
        Ok(_) => {
            HttpResponse::Ok().json("Driver location updated successfully")
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

