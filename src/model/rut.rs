// rut model

use db::schema::ruts;
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::{ Msgs, RutMsgs, RutListMsgs };

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
    pub user_id: String,      // as who post
    pub credential: String,
    pub item_count: i32,
    pub comment_count: i32,
    pub star_count: i32,
}

// use to build insert query
#[derive(Serialize,Deserialize,Insertable,Debug,Clone)]
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
    pub credential: &'a str,
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
    pub credential: String,
}

impl Message for CreateRut {
    type Result = Result<RutMsgs, Error>;
}

// as msg in select by id
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RutID {
    pub rut_id: String,
}

impl Message for RutID {
    type Result = Result<RutMsgs, Error>;
}

// as msg in select rutlist
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RutListType {
    Index(String),
    UserID(String),
    ItemID(String),
}

impl Message for RutListType {
    type Result = Result<RutListMsgs, Error>;
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
            author_id: "".to_owned(),
            credential: "".to_owned(),
            item_count: 0,
            comment_count: 0,
            star_count: 0,
        }
    }
}
