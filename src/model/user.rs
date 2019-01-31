// user model

use db::schema::users;
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::Msgs;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Identifiable, Queryable)]
#[table_name="users"]
pub struct User {
    pub id: String,
    pub uname: String,
    pub password: String,
    pub join_at: NaiveDateTime,
    pub avatar: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub uname: &'a str,
    pub password: &'a str,
    pub join_at: NaiveDateTime,
    pub avatar: &'a str,
}

// message to create user
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUser {
    pub uname: String,
    pub password: String,
    pub confirm_password: String,
}

impl Message for CreateUser {
    type Result = Result<Msgs, Error>;
}