// item module

use db::schema::{items, collects};
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::{ Msgs, ItemMsgs, ItemListMsgs, CollectMsgs };

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
    pub cover: String,    // img url
    pub edition: String,  // binding, version ...
    pub detail: String,
    pub rut_count: i32,
    pub etc_count: i32,   // review, etc.
    pub done_count: i32,  // num of who done
}

// use to build insert query
#[derive(Debug,Clone,Serialize,Deserialize,Insertable)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub uiid: &'a str,  // unique item id, like isbn...
    pub pub_at: &'a str,  // "MM-DD-YYYY"
    pub authors: &'a str,
    pub publisher: &'a str,
    pub category: &'a str, // Book or Cource ...
    pub url: &'a str,
    pub cover: &'a str,    // img url
    pub edition: &'a str,  // binding, version ...
    pub detail: &'a str,
    pub rut_count: i32,
    pub etc_count: i32,   // review, etc.
    pub done_count: i32,  // num of who done
}

// as msg in create new
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SubmitItem {
    pub title: String,
    pub uiid: String,  // unique item id, like isbn...
    pub pub_at: String,  // "MM-DD-YYYY"
    pub authors: String,
    pub publisher: String,
    pub category: String, // Book or Cource ...
    pub url: String,
    pub cover: String,    // img url
    pub edition: String,  // binding, version ...
    pub detail: String,
}

impl Message for SubmitItem {
    type Result = Result<ItemMsgs, Error>;
}

// as msg in query item by id
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemID {
    pub item_id: String,
}

impl Message for ItemID {
    type Result = Result<ItemMsgs, Error>;
}

// as msg in query item list by id, uiid, title, ..
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ItemIDs {
    ID(String),
    Uiid(String),
    Title(String),
    Url(String),
}

impl Message for ItemIDs {
    type Result = Result<ItemListMsgs, Error>;
}

// Item's constructor
impl Item {
    pub fn new() -> Self {
        Item {
            id: "".to_owned(),
            title: "".to_owned(),
            uiid: "".to_owned(),   
            pub_at: "".to_owned(),   
            authors: "".to_owned(),
            publisher: "".to_owned(),
            category: "".to_owned(), 
            url: "".to_owned(),
            cover: "".to_owned(),    
            edition: "".to_owned(),  
            detail: "".to_owned(),
            rut_count: 0,
            etc_count: 0,   
            done_count: 0,
        }
    }
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="collects"]
pub struct Collect {
    pub id: String,
    pub rut_id: String,
    pub item_id: String,
    pub item_order: i32,
    pub content: String,
    // pub spoiler: bool,  // to do
    pub creator_id: String,
    pub collect_at: NaiveDateTime,
}

#[derive(Debug,Clone,Serialize,Deserialize,Insertable)]
#[table_name="collects"]
pub struct NewCollect<'a> {
    pub id: &'a str,
    pub rut_id: &'a str,
    pub item_id: &'a str,
    pub item_order: i32,
    pub content: &'a str,
    // pub spoiler: bool,  // to do
    pub creator_id: &'a str,
    pub collect_at: NaiveDateTime,
}

// as msg in rut collect new item
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CollectItem {
    pub rut_id: String,
    pub item_id: String,
    pub item_order: i32,
    pub content: String,
    // pub spoiler: bool,  // to do
    pub creator_id: String,
}

impl Message for CollectItem {
    type Result = Result<CollectMsgs, Error>;
}
