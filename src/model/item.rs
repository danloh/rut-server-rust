// item module

use db::schema::{items, collects};
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};

// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="items"]
pub struct Item {
    pub id: String,
    pub title: String,
    pub uiid: String,  // unique item id, like isbn...
    pub pub_at: String,  // "MM-DD-YYYY"
    pub authors: String,
    pub publisher: String,
    pub category: String, // Book or Cource ...
    pub url: String,
    pub cover: String,    //  img url
    pub edition: String,  // binding, version ...
    pub detail: String,
    pub rut_count: i32,
    pub etc_count: i32,   // review, etc.
    pub done_count: i32,  // num of who done
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="collects"]
pub struct Collect {
    pub id: String,
    pub rut_id: String,
    pub item_id: String,
    pub item_order: i32,
    pub content: String,
    pub creator_id: String,
    pub collect_at: NaiveDateTime,
}
