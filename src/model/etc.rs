// etc module: excerpt, article, review, comment, etc.

use db::schema::etcs;
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::{ Msg, EtcMsg, EtcListMsg };

// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="etcs"]
pub struct Etc {
    pub id: String,
    pub content: String,
    pub post_at: NaiveDateTime,
    pub petc_id: String,  // e.g. comment a comment
    pub rut_id: String,
    pub item_id: String,
    pub tname: String, 
    pub uname: String,  // who post
    pub vote: i32,
}

// use to build insert query
#[derive(Debug,Clone,Serialize,Deserialize,Insertable)]
#[table_name="etcs"]
pub struct NewEtc<'a> {
    pub id: &'a str,
    pub content: &'a str,
    pub post_at: NaiveDateTime,
    pub petc_id: &'a str,
    pub rut_id: &'a str,
    pub item_id: &'a str,
    pub tname: &'a str,
    pub uname: &'a str,
    pub vote: i32,
}

// as msg in create new
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostEtc {
    pub content: String,
    pub post_to: String,
    pub to_id: String,
    pub uname: String,
}

impl Message for PostEtc {
    type Result = Result<EtcMsg, Error>;
}

// as msg to get etc list
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EtcsPerID {     // diff way from enum to get
    pub per: String,
    pub per_id: String,
    pub paging: i32,
}

impl Message for EtcsPerID {
    type Result = Result<EtcListMsg, Error>;
}

// as msg to del etc
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DelEtc {
    pub etc_id: String,
    pub rut_id: String,   // to update rut after del
    pub item_id: String,  // to update item after del
    pub uname: String,  // to check permission
}

impl Message for DelEtc {
    type Result = Result<Msg, Error>;
}
