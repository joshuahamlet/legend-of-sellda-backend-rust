use std::env;
use dotenv::dotenv; 

use mongodb::sync::{Client, Collection};

use crate::models::user_model::User;
use crate::models::product_model::Product;

pub struct MongoRepo {
    pub(crate) users: Collection<User>,
    pub(crate) products: Collection<Product>
}

impl MongoRepo {
    pub fn init() -> Self {
    dotenv().ok();
            let uri = match env::var("MONGOURI") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variable"),
            };
            let client = Client::with_uri_str(uri).unwrap();
            let db = client.database("LegendofSellda");
            let users: Collection<User> = db.collection("users");
            let products: Collection<Product> = db.collection("products");
            MongoRepo { users, products }
    }
}

