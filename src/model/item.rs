// item module

use db::schema::{items, collects, staritems};
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::{ Msg, ItemMsg, ItemListMsg, CollectMsg, CollectsMsg };

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
    // pub action: String, // get / delete, to do
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
    pub user_id: String,
    pub collect_at: NaiveDateTime,
}

// Collect's constructor
impl Collect {
    pub fn new() -> Self {
        Collect {
            id: "".to_owned(),
            rut_id: "".to_owned(),
            item_id: "".to_owned(),
            item_order: 0,
            content: "".to_owned(), 
            user_id: "".to_owned(),   
            collect_at: Utc::now().naive_utc(),
        }
    }
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

// as msg in update item
#[derive(Deserialize,Serialize,Debug,Clone,AsChangeset)]
#[table_name="collects"]
pub struct UpdateCollect {
    pub id: String,
    // pub item_order: i32,  // re-order, to do
    pub content: String,
    pub user_id: String,  // to check permission
    // pub spoiler: bool,  // to do but 
}

impl Message for UpdateCollect {
    type Result = Result<CollectMsg, Error>;
}

// as msg in rut get collect info
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CollectID {
    pub collect_id: String,
    pub action: String,    // get / delete
}

impl Message for CollectID {
    type Result = Result<CollectMsg, Error>;
}

// as msg to del collect
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DelCollect {
    pub collect_id: String,
    pub rut_id: String,   // to update rut after del
    pub item_id: String,  // to update item after del
    pub user_id: String,  // to check permission
}

impl Message for DelCollect {
    type Result = Result<Msg, Error>;
}

// as msg in collect list per rutid or itemid
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CollectIDs {
    RutID(String),
    ItemID(String),
    UserID(String),
}

impl Message for CollectIDs {
    type Result = Result<CollectsMsg, Error>;
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="staritems"]
pub struct StarItem {
    pub id: String,
    pub user_id: String,
    pub rut_id: String,
    pub star_at: NaiveDateTime,
    pub note: String,
    pub flag: String,    // 0->todo,1->doing,2->done
}
