// item module

use db::schema::{items, collects};
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::{ Msg, ItemMsg, ItemListMsg, CollectMsg };

// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="items"]
pub struct Item {
    pub id: String,
    pub title: String,
    pub uiid: String,  // unique item id, like isbn...
    pub authors: String,
    pub pub_at: String,  // "MM-DD-YYYY
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
    pub authors: &'a str,
    pub pub_at: &'a str,  // "MM-DD-YYYY"
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

// as msg in submit new item
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SubmitItem {
    pub title: String,
    pub uiid: String,  // unique item id, like isbn...
    pub authors: String,
    pub pub_at: String,  // "MM-DD-YYYY"
    pub publisher: String,
    pub category: String, // Book or Cource ...
    pub url: String,
    pub cover: String,    // img url
    pub edition: String,  // binding, version ...
    pub detail: String,
}

impl Message for SubmitItem {
    type Result = Result<ItemMsg, Error>;
}

// as msg in update item
#[derive(Deserialize,Serialize,Debug,Clone,AsChangeset)]
#[table_name="items"]
pub struct UpdateItem {
    pub id: String,
    pub title: String,
    pub uiid: String,
    pub authors: String,
    pub pub_at: String, 
    pub publisher: String,
    pub category: String,
    pub url: String,
    pub cover: String,
    pub edition: String,
    pub detail: String,
}

impl Message for UpdateItem {
    type Result = Result<ItemMsg, Error>;
}

// as msg in query item by id
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemID {
    pub item_id: String,
}

impl Message for ItemID {
    type Result = Result<ItemMsg, Error>;
}

// as msg to query items per tag, rut, user; id,title,url,uiid
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ItemsPerID {
    ItemID(String),
    Uiid(String),
    Title(String),
    ItemUrl(String),
    RutID(String),
    TagID(String),
    // UserID(String, String),  // (userid, flag)
}

impl Message for ItemsPerID {
    type Result = Result<ItemListMsg, Error>;
}

// Item's constructor
impl Item {
    pub fn new() -> Self {
        Item {
            id: "".to_owned(),
            title: "".to_owned(),
            uiid: "".to_owned(),
            authors: "".to_owned(), 
            pub_at: "".to_owned(),   
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
    // pub spoiler: bool,  // to do but 
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
    pub user_id: &'a str,
    pub collect_at: NaiveDateTime,
}

// as msg in rut collect new item
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CollectItem {
    pub rut_id: String,
    pub item_id: String,
    pub item_order: i32,
    pub content: String,
    pub user_id: String,
}

impl Message for CollectItem {
    type Result = Result<CollectMsg, Error>;
}

// as msg in rut get collect info
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CollectID {
    pub rut_id: String,
    pub item_id: String,
}

impl Message for CollectID {
    type Result = Result<CollectMsg, Error>;
}
