// get driver location using websocket and send to rider and also store in redis

use actix_web::{web, HttpResponse, Responder};
use actix_web_actors::ws;

use crate::models::driver_model::Driver;
use crate::config::AppState;

pub async fn get_driver_location(driver: Driver, data: web::Data<AppState>) -> impl Responder {
    let mut redis_conn = data.redis_client.get_async_connection().await.unwrap();
    redis_conn.set_ex(format!("driver:{}", driver.id), driver.current_location, 3600).await.unwrap();
    Ok(())
}