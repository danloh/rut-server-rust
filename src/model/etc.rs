// etc module: excerpt, article, review, comment, etc.

use db::schema::etcs;
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};

// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="etcs"]
pub struct Etc {
    pub id: String,
    pub content: String,
    pub create_at: NaiveDateTime,
    pub etc_id: String, // e.g. comment a comment
}
