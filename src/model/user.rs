// user model

use db::schema::users;
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::{ Msgs, LoginMsgs };

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
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

// message to sign up user
#[derive(Deserialize, Serialize, Debug)]
pub struct SignUser {
    pub uname: String,
    pub password: String,
    pub confirm_password: String,
}

impl Message for SignUser {
    type Result = Result<Msgs, Error>;
}

// message to login user
#[derive(Deserialize, Serialize, Debug)]
pub struct LogUser {
    pub uname: String,
    pub password: String,
}

impl Message for LogUser {
    type Result = Result<LoginMsgs, Error>;
}

// User's constructor
impl User {
    pub fn new() -> User {
        User {
            id: "".to_owned(),
            uname: "".to_owned(),
            password: "".to_owned(),
            join_at: Utc::now().naive_utc(),
            avatar: "".to_owned(),
        }
    }
}
