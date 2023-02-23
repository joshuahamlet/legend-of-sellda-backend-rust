use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)] 
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub image: String,
    pub price: i32,
    pub rating: i32,
    pub numreviews: i32,
    #[serde(rename = "numberInStock")]
    pub number_in_stock: i32,
    #[serde(rename = "productType")]
    pub product_type: String,
    pub description: String
}
