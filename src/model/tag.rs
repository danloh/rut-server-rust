// tag typed model and msg handler

use actix::{ Message };
use chrono::{ NaiveDateTime };

use crate::errors::ServiceError;
use crate::model::msg::{ Msg, TagMsg, TagListMsg };
use crate::schema::{ tags, tagruts, tagitems, tagetcs, startags };


// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
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

// Rut's constructor
impl Tag {
    pub fn new(tname: String) -> Self {
        Tag {
            id: tname.clone(),
            tname: tname.clone(),
            intro: "".to_owned(),
            logo: "".to_owned(),
            pname: "".to_owned(),
            item_count: 0,
            rut_count: 0,
            etc_count: 0,
            star_count: 0,
            vote: 0,
        }
    }
}

// as msg in create new tag, get tag
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CheckTag {
    pub tname: String,
    pub action: String, // get / post / delete
}

impl Message for CheckTag {
    type Result = Result<TagMsg, ServiceError>;
}

// as msg in query tag list
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum QueryTags {
    RutID(String),
    ItemID(String),
    TagID(String),
    UserID(String),
    Index(String),
}

impl Message for QueryTags {
    type Result = Result<TagListMsg, ServiceError>;
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
    type Result = Result<TagMsg, ServiceError>;
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="tagruts"]
pub struct TagRut {
    pub id: String,
    pub tname: String,
    pub rut_id: String,
    pub count: i32,
}

// as msg in tag or untag rut
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RutTag {
    pub tnames: Vec<String>,
    pub rut_id: String,
    pub action: u8, // tag 1 or untag 0
}

impl Message for RutTag {
    type Result = Result<Msg, ServiceError>;
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="startags"]
pub struct StarTag {
    pub id: String,
    pub uname: String,
    pub tname: String,
    pub star_at: NaiveDateTime,
    pub note: String,
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
    type Result = Result<Msg, ServiceError>;
}

// as msg to check if star a tag
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarTagStatus {
    pub uname: String,
    pub tname: String,
}

impl Message for StarTagStatus {
    type Result = Result<Msg, ServiceError>;
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
