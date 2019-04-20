// etc typed model and msg handler

use actix::{ Message };
use chrono::{ NaiveDateTime };

use crate::errors::ServiceError;
use crate::model::msg::{ Msg, EtcMsg, EtcListMsg };
use crate::schema::etcs;


// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="etcs"]
pub struct Etc {
    pub id: String,
    pub content: String,
    pub post_at: NaiveDateTime,
    pub petc_id: String,  // e.g. comment a comment
    pub rut_id: String,
    pub item_id: String,
    pub tname: String, 
    pub uname: String,  // who post
    pub vote: i32,
}

// as msg in create new
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct PostEtc {
    pub content: String,
    pub post_to: String,
    pub to_id: String,
    pub uname: String,
}

impl Message for PostEtc {
    type Result = Result<EtcMsg, ServiceError>;
}

// as msg to get etc list
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryEtcs {     // diff way from enum to get
    pub per: String,
    pub perid: String,
    pub page: i32,
}

impl Message for QueryEtcs {
    type Result = Result<EtcListMsg, ServiceError>;
}


// todo
// as msg to del etc
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DelEtc {
    pub etc_id: String,
    pub rut_id: String,   // to update rut after del
    pub item_id: String,  // to update item after del
    pub uname: String,  // to check permission
}

impl Message for DelEtc {
    type Result = Result<Msg, ServiceError>;
}
