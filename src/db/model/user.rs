// user model

use db::schema::users;
use actix_web::{ Error, actix::Message };

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

// message to create user
pub struct CreateUser {
    pub name: String,
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}