// tag module 

use db::schema::{ tags, tagruts, tagitems, tagetcs, startags };
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::{ Msg, TagMsg, TagListMsg };

// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="tags"]
pub struct Tag {
    pub id: String,
    pub tname: String,
    pub intro: String,
    pub logo: String,
    pub pname: String,  // parent tag name != tname. check constain
    pub item_count: i32,
    pub rut_count: i32,
    pub etc_count: i32,
    pub star_count: i32,
    pub vote: i32,       //cal per star,rut,item,comment
}

// use to build insert query
#[derive(Debug,Clone,Serialize,Deserialize,Insertable)]
#[table_name="tags"]
pub struct NewTag<'a> {
    pub id: &'a str,
    pub tname: &'a str,
    pub intro:&'a str,
    pub logo: &'a str,
    pub pname: &'a str,  // parent tag name
    pub item_count: i32,
    pub rut_count: i32,
    pub etc_count: i32,
    pub star_count: i32,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="tagruts"]
pub struct TagRut {
    pub id: String,
    pub tname: String,
    pub rut_id: String,
    pub count: i32,
}

// use to build insert query
#[derive(Debug,Clone,Serialize,Deserialize,Insertable)]
#[table_name="tagruts"]
pub struct NewTagRut<'a> {
    pub id: &'a str,
    pub tname: &'a str,
    pub rut_id: &'a str,
    pub count: i32,
}

// as msg in tag or untag rut
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RutTag {
    pub tnames: Vec<String>,
    pub rut_id: String,
    pub action: String, // tag or untag
}

impl Message for RutTag {
    type Result = Result<Msg, Error>;
}

// to do
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="tagitems"]
pub struct TagItem {
    pub id: String,
    pub tname: String,
    pub item_id: String,
    pub count: i32,
}

// to do
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="tagetcs"]
pub struct TagEtc {
    pub id: String,
    pub tname: String,
    pub etc_id: String,
}

// as msg in create new tag, get tag
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CheckTag {
    pub tname: String,
    pub action: String, // get / post / delete
}

impl Message for CheckTag {
    type Result = Result<TagMsg, Error>;
}

// as msg in update tag
#[derive(Deserialize,Serialize,Debug,Clone,AsChangeset)]
#[table_name="tags"]
pub struct UpdateTag {
    pub tname: String,
    pub intro: String,
    pub logo: String,
    pub pname: String,  // parent tag name
}

impl Message for UpdateTag {
    type Result = Result<TagMsg, Error>;
}

// as msg in query tag list
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TagsPerID {
    RutID(String),
    ItemID(String),
    TagID(String),
    UserID(String),
    Index(String),
}

impl Message for TagsPerID {
    type Result = Result<TagListMsg, Error>;
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="startags"]
pub struct StarTag {
    pub id: String,
    pub uname: String,
    pub tname: String,
    pub star_at: NaiveDateTime,
    pub note: String,
}

// use to build insert query
#[derive(Debug,Clone,Serialize,Deserialize,Insertable)]
#[table_name="startags"]
pub struct TagStar<'a> {
    pub id: &'a str,
    pub uname: &'a str,
    pub tname: &'a str,
    pub star_at: NaiveDateTime,
    pub note: &'a str,
}

// as msg in star or unstar tag
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarOrTag {
    pub uname: String,
    pub tname: String,
    pub note: String,
    pub action: u8,  // 0- unstar, 1- star
}

impl Message for StarOrTag {
    type Result = Result<Msg, Error>;
}

// as msg to check if star a tag
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarTagStatus {
    pub uname: String,
    pub tname: String,
}

impl Message for StarTagStatus {
    type Result = Result<Msg, Error>;
}
