// item module

use db::schema::{items, collects, staritems};
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::{ Msg, ItemMsg, ItemListMsg, StarItemMsg, CollectMsg, CollectsMsg };

// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
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
    pub vote: i32,        //  cal per rut, done, etc
    pub slug: String, 
}

// Item's constructor
impl Item {
    pub fn new(id: String, slug: String, item: SubmitItem) -> Self {
        Item {
            id,
            title: item.title,
            uiid: item.uiid,
            authors: item.authors, 
            pub_at: item.pub_at,   
            publisher: item.publisher,
            category: item.category, 
            url: item.url,
            cover: item.cover,    
            edition: item.edition,  
            detail: item.detail,
            rut_count: 0,
            etc_count: 0,   
            done_count: 0,
            vote: 0,
            slug,
        }
    }
}

// as msg in submit new item
#[derive(Deserialize,Serialize,Debug,Clone)]
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
    UserID(String, String, i32),  // (uname, flag, paging)
    KeyID(String, String, String, i32) // keyword, per, perid(uname|tname), paging
}

impl Message for ItemsPerID {
    type Result = Result<ItemListMsg, Error>;
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="collects"]
pub struct Collect {
    pub id: String,
    pub rut_id: String,
    pub item_id: String,
    pub item_order: i32,
    pub content: String,
    // pub spoiler: bool,  // to do but 
    pub uname: String,
    pub collect_at: NaiveDateTime,
}

// Collect's constructor
impl Collect {
    pub fn new(uid: String, c: CollectItem) -> Self {
        Collect {
            id: uid,
            rut_id: c.rut_id,
            item_id: c.item_id,
            item_order: c.item_order,
            content: c.content, 
            uname: c.uname,   
            collect_at: Utc::now().naive_utc(),
        }
    }
}

// as msg in rut collect new item
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CollectItem {
    pub rut_id: String,
    pub item_id: String,
    pub item_order: i32,
    pub content: String,
    pub uname: String,
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
    pub uname: String,  // to check permission
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
    pub uname: String,  // to check permission
}

impl Message for DelCollect {
    type Result = Result<Msg, Error>;
}

// as msg in collect list per rutid or itemid
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CollectIDs {
    RutID(String),
    ItemID(String, i32),  // id, paging
    UserID(String, i32),  // id, paging
}

impl Message for CollectIDs {
    type Result = Result<CollectsMsg, Error>;
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="staritems"]
pub struct StarItem {
    pub id: String,
    pub uname: String,
    pub item_id: String,
    pub star_at: NaiveDateTime,
    pub note: String,
    pub flag: String,    // 0->to do,1->done, 2->doing
    pub rate: i32,
}

// as msg in star item: todo, done, doing
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewStarItem {
    pub uname: String,
    pub item_id: String,
    pub note: String,
    pub flag: String,
    pub rate: i32,
}

impl Message for NewStarItem {
    type Result = Result<StarItemMsg, Error>;
}

// as msg to check if star a rut
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarItemStatus {
    pub uname: String,
    pub item_id: String,
}

impl Message for StarItemStatus {
    type Result = Result<StarItemMsg, Error>;
}
