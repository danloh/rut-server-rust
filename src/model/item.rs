// item typed model and msg handler

use actix::{ Message };
use chrono::{ NaiveDateTime, Utc };
use actix_web::{ Error, error };

use crate::errors::ServiceError;
use crate::util::share::{ gen_slug };
use crate::model::{
    Validate, test_len_limit, re_test_url,
    TITLE_LEN, UIID_LEN,
};
use crate::model::msg::{
    Msg, ItemMsg, ItemListMsg, StarItemMsg, CollectMsg, CollectsMsg
};
use crate::schema::{ items, collects, staritems };

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
    pub slug: String, // to do
}

// Item's constructor
impl Item {
    pub fn new(uid: String, slug: String, item: NewItem) -> Self {
        Item {
            id: uid,
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
pub struct NewItem {
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

// Item's constructor
impl NewItem {
    pub fn new() -> Self {
        NewItem {
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
        }
    }
}

impl Message for NewItem {
    type Result = Result<ItemMsg, ServiceError>;
}

impl Validate for NewItem {
    fn validate(&self) -> Result<(), Error> {
        let url = &self.url.trim();
        let cover = &self.cover.trim();
        let url_test = if url.len() == 0 { true } else { re_test_url(url) };
        let cover_test = if cover.len() == 0 { true } else { re_test_url(cover) };
        let check_len =
            test_len_limit(&self.title, 3, TITLE_LEN) &&
            test_len_limit(&self.uiid, 0, 32) &&
            test_len_limit(&self.authors, 1, 128) &&
            test_len_limit(&self.pub_at, 0, 32) &&
            test_len_limit(&self.publisher, 0, 64) &&
            test_len_limit(&self.category, 0, 32) &&
            test_len_limit(&self.edition, 0, 64);
        let check = url_test && cover_test && check_len;

        if check {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("Invalid Input"))
        }
    }
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
    type Result = Result<ItemMsg, ServiceError>;
}

impl Validate for UpdateItem {
    fn validate(&self) -> Result<(), Error> {
        let url = &self.url.trim();
        let cover = &self.cover.trim();
        let url_test = if url.len() == 0 { true } else { re_test_url(url) };
        let cover_test = if cover.len() == 0 { true } else { re_test_url(cover) };
        let check_len =
            test_len_limit(&self.id, 8, 512) &&
            test_len_limit(&self.title, 3, TITLE_LEN) &&
            test_len_limit(&self.uiid, 0, 32) &&
            test_len_limit(&self.authors, 1, 128) &&
            test_len_limit(&self.pub_at, 0, 32) &&
            test_len_limit(&self.publisher, 0, 64) &&
            test_len_limit(&self.category, 0, 32) &&
            test_len_limit(&self.edition, 0, 64);
        let check = url_test && cover_test && check_len;

        if check {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("Invalid Input"))
        }
    }
}

// as msg in query item by slug
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct QueryItem {
    pub item_slug: String,
    // pub action: String, // get / delete, to do
}

impl Message for QueryItem {
    type Result = Result<ItemMsg, ServiceError>;
}

// as msg to query items per tag, rut, user; id,title,url,uiid
#[derive(Deserialize,Serialize,Debug,Clone)]
pub enum QueryItems {
    ItemID(String), // should be slug
    Uiid(String),
    Title(String),
    ItemUrl(String),
    RutID(String),
    TagID(String),
    UserID(String, i16, i32),  // (uname, flag, paging)
    KeyID(String, String, String, i32) // keyword, per, perid(uname|tname), paging
}

impl Message for QueryItems {
    type Result = Result<ItemListMsg, ServiceError>;
}

impl Validate for QueryItems  {
    fn validate(&self) -> Result<(), Error> {
        let check: bool = match self {
            QueryItems::ItemUrl(url) => { re_test_url(url) },
            // could do more
            _ => { true },
        };

        if check {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("Invalid Input"))
        }
    }
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="collects"]
pub struct Collect {
    pub id: String,
    pub rut_id: String,
    pub item_id: String,
    pub item_order: i16,
    pub content: String,
    // pub spoiler: bool,  // to do but
    pub uname: String,
    pub collect_at: NaiveDateTime,
}

// Collect's constructor
impl Collect {
    pub fn new(uid: String, i_order: i16, c: CollectItem) -> Self {
        Collect {
            id: uid,
            rut_id: c.rut_id,
            item_id: c.item_id,
            item_order: i_order,
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
    pub item_order: i16,
    pub content: String,
    pub uname: String,
}

impl Message for CollectItem {
    type Result = Result<CollectMsg, ServiceError>;
}

// as msg in update item
#[derive(Deserialize,Serialize,Debug,Clone,AsChangeset)]
#[table_name="collects"]
pub struct UpdateCollect {
    pub id: String,
    // pub item_order: i16,  // re-order, to do
    pub content: String,
    pub uname: String,  // to check permission
    // pub spoiler: bool,  // to do but
}

impl Message for UpdateCollect {
    type Result = Result<CollectMsg, ServiceError>;
}

// as msg to del collect
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DelCollect {
    pub collect_id: String,
    pub uname: String,  // to check permission
}

impl Message for DelCollect {
    type Result = Result<Msg, ServiceError>;
}

// as msg in rut get collect info
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryCollect {
    pub collect_id: String,
    pub action: String,    // get|delete
}

impl Message for QueryCollect {
    type Result = Result<CollectMsg, ServiceError>;
}

// as msg in collect list per rutid or itemid
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum QueryCollects {
    RutID(String),
    ItemID(String, i32),  // id, paging
    UserID(String, i32),  // id, paging
}

impl Message for QueryCollects {
    type Result = Result<CollectsMsg, ServiceError>;
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="staritems"]
pub struct StarItem {
    pub id: String,
    pub uname: String,
    pub item_id: String,
    pub star_at: NaiveDateTime,
    pub note: String,
    pub flag: i16,    // 1-Todo|3-Done|2-Doing
    pub rate: i16,
}

// as msg in star item: todo, done, doing
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewStarItem {
    pub uname: String,
    pub item_id: String,
    pub note: String,
    pub flag: i16,
    pub rate: i16,
}

impl Message for NewStarItem {
    type Result = Result<StarItemMsg, ServiceError>;
}

// as msg to check if star a rut
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarItemStatus {
    pub uname: String,
    pub item_id: String,
}

impl Message for StarItemStatus {
    type Result = Result<StarItemMsg, ServiceError>;
}
