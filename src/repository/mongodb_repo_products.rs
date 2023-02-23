use mongodb::bson::{doc, extjson::de::Error, oid::ObjectId};

use crate::models::product_model::Product;
use crate::repository::mongodb_repo::MongoRepo;

impl MongoRepo {

    pub fn get_products(&self) -> Result<Vec<Product>, Error> {
        let cursors = self
            .products
            .find(None, None)
            .ok()
            .expect("Error Getting Product details");
        let product_details = cursors.map(|doc| doc.unwrap()).collect();
        Ok(product_details)
    }

    pub fn get_product(&self, id: &String) -> Result<Product, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let product_detail = self
            .products
            .find_one(filter, None)
            .ok()
            .expect("Error Getting a Product's detail");
        Ok(product_detail.unwrap())
    }
}
