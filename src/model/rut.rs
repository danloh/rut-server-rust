// rut model

use db::schema::ruts;
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::Msgs;

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable,QueryableByName)]
#[table_name = "ruts"]
pub struct Rut {
    pub id: String,
    pub title: String,
    pub url: String,
    pub content: String,
    pub create_at: NaiveDateTime,
    pub user_id: String,
    pub user_intro: String,
    pub item_count: i32,
    pub comment_count: i32,
    pub star_count: i32,
}

#[derive(Serialize,Deserialize,Insertable,Debug,Clone)]
#[table_name="ruts"]
pub struct NewRut<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub url: &'a str,
    pub content: &'a str,
    pub create_at: NaiveDateTime,
    pub user_id: &'a str,
    pub user_intro: &'a str,
    pub item_count: i32,
    pub comment_count: i32,
    pub star_count: i32,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct CreateRut {
    pub title: String,
    pub url: String,
    pub content: String,
    pub user_id: String,
    pub user_intro: String,
}

impl Message for CreateRut {
    type Result = Result<Msgs, Error>;
}
