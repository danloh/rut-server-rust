// rut model

use db::schema::{ ruts, starruts };
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::{ Msg, RutMsg, RutListMsg };

// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="ruts"]
pub struct Rut {
    pub id: String,
    pub title: String,
    pub url: String,
    pub content: String,
    pub create_at: NaiveDateTime,
    pub renew_at: NaiveDateTime,
    pub author_id: String,
    pub user_id: String,     // as who post
    pub user_name: String,
    pub credential: String,
    pub logo: String,
    pub item_count: i32,
    pub comment_count: i32,
    pub star_count: i32,
    // pub vote: i32,  // to do, cal per star, comment
}

// use to build insert query
#[derive(Debug,Clone,Serialize,Deserialize,Insertable)]
#[table_name="ruts"]
pub struct NewRut<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub url: &'a str,
    pub content: &'a str,
    pub create_at: NaiveDateTime,
    pub renew_at: NaiveDateTime,
    pub author_id: &'a str,
    pub user_id: &'a str,
    pub user_name: &'a str,
    pub credential: &'a str,
    pub logo: &'a str,
    pub item_count: i32,
    pub comment_count: i32,
    pub star_count: i32,
}

// as msg in create new
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateRut {
    pub title: String,
    pub url: String,
    pub content: String,
    pub user_id: String,
    pub user_name: String,
    pub author_id: String,
    pub credential: String,
}

impl Message for CreateRut {
    type Result = Result<RutMsg, Error>;
}

// as msg in update rut
#[derive(Deserialize,Serialize,Debug,Clone,AsChangeset)]
#[table_name="ruts"]
pub struct UpdateRut {
    pub id: String,
    pub title: String,
    pub url: String,
    pub content: String,
    pub author_id: String,
    pub credential: String,
}

impl Message for UpdateRut {
    type Result = Result<RutMsg, Error>;
}

// as msg in select by id
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RutID {
    pub rut_id: String,
}

impl Message for RutID {
    type Result = Result<RutMsg, Error>;
}

// as msg to get  rut list
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RutsPerID {
    Index(String),
    UserID(String, String),
    ItemID(String),
    TagID(String),
}

impl Message for RutsPerID {
    type Result = Result<RutListMsg, Error>;
}

// Rut's constructor
impl Rut {
    pub fn new() -> Rut {
        Rut {
            id: "".to_owned(),
            title: "".to_owned(),
            url: "".to_owned(),
            content: "".to_owned(),
            create_at: Utc::now().naive_utc(),
            renew_at: Utc::now().naive_utc(),
            user_id: "".to_owned(),
            user_name: "".to_owned(),
            author_id: "".to_owned(),
            credential: "".to_owned(),
            logo: "".to_owned(),
            item_count: 0,
            comment_count: 0,
            star_count: 0,
        }
    }
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="starruts"]
pub struct StarRut {
    pub id: String,
    pub user_id: String,
    pub rut_id: String,
    pub star_at: NaiveDateTime,
    pub note: String,
}

// use to build insert query
#[derive(Debug,Clone,Serialize,Deserialize,Insertable)]
#[table_name="starruts"]
pub struct RutStar<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub rut_id: &'a str,
    pub star_at: NaiveDateTime,
    pub note: &'a str,
}

// as msg in star or unstar rut
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarOrRut {
    pub rut_id: String,
    pub user_id: String,
    pub note: String,
    pub action: u8,  // 0- unstar, 1- star
}

impl Message for StarOrRut {
    type Result = Result<Msg, Error>;
}

// as msg to check if star a rut
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarRutStatus {
    pub user_id: String,
    pub rut_id: String,
}

impl Message for StarRutStatus {
    type Result = Result<Msg, Error>;
}
