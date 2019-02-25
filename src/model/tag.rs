// tag module 

use db::schema::{ tags, tagruts, tagitems, tagetcs, startags };
use actix_web::{ Error, actix::Message };

// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="tags"]
pub struct Tag {
    pub id: String,
    pub tname: String,
    pub intro: String,
    pub logo: String,
    pub parent_id: String,  // parent tag
    pub item_count: i32,
    pub rut_count: i32,
    pub etc_count: i32,
    pub star_count: i32,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="tagruts"]
pub struct TagRut {
    pub id: String,
    pub tag_id: String,
    pub rut_id: String,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="tagitems"]
pub struct TagItem {
    pub id: String,
    pub tag_id: String,
    pub item_id: String,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="tagetcs"]
pub struct TagEtc {
    pub id: String,
    pub tag_id: String,
    pub etc_id: String,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="startags"]
pub struct StarTag {
    pub id: String,
    pub user_id: String,
    pub tag_id: String,
}
