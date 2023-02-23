use crate::{models::product_model::Product, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};

#[get("/product/<path>")]
pub fn get_product(db: &State<MongoRepo>, path: String) -> Result<Json<Product>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let product_detail = db.get_product(&id);
    match product_detail {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/product")]
pub fn get_products(db: &State<MongoRepo>) -> Result<Json<Vec<Product>>, Status> {
    let product_details = db.get_products();
    match product_details {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(Status::InternalServerError),
    }
}
