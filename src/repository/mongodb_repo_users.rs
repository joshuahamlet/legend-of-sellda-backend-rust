use mongodb::{
    bson::{
        extjson::de::Error, 
        oid::ObjectId, 
        doc
    },
    results::InsertOneResult,
};

use crate::models::user_model::User;
use crate::repository::mongodb_repo::MongoRepo;

impl MongoRepo {

    #[allow(dead_code)]
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
       let new_doc = User {
                id: None,
                name: new_user.name,
            password: new_user.password,
            email: new_user.email,
            date: new_user.date,
            is_admin: new_user.is_admin

            };
            let user = self
                .users
                .insert_one(new_doc, None)
                .ok()
                .expect("Error creating user");
            Ok(user)
        
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .users
            .find_one(filter, None)
            .ok()
            .expect("Error Getting a User's detail");
        Ok(user_detail.unwrap())
    }
}

