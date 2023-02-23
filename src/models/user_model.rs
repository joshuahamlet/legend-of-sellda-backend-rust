use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)] 
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String, 
    pub date: DateTime,
    pub password: String,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool
}
