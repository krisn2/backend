use mongodb::{Client, Collection};

use crate::models::driver_model::Driver;
use crate::models::user_model::User;
use crate::models::ride_model::Trip;

pub struct AppState {
    pub mongo_client: Client,
    pub redis_client: redis::Client,
    pub driver_collection: Collection<Driver>,
    pub rider_collection: Collection<User>,
    pub trip_collection: Collection<Trip>,
}

impl AppState {
    pub async fn new(mongo_uri: &str, redis_uri: &str) -> Self {
        let mongo_client = Client::with_uri_str(mongo_uri).await.unwrap();
        let redis_client = redis::Client::open(redis_uri).unwrap();
        let driver_collection = mongo_client.database("uber").collection("drivers");
        let rider_collection = mongo_client.database("uber").collection("riders");
        let trip_collection = mongo_client.database("uber").collection("trips");
        Self {
            mongo_client,
            redis_client,
            driver_collection,
            rider_collection,
            trip_collection,
        }
    }
}