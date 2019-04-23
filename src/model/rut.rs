// rut typed model and msg handler

use actix::{ Message };
use chrono::{ NaiveDateTime, Utc };
use actix_web::{ Error, error };

use crate::errors::ServiceError;
use crate::model::{ Validate, test_len_limit, re_test_url, TITLE_LEN };
use crate::model::msg::{ Msg, RutMsg, RutListMsg, StarStatusMsg };
use crate::schema::{ ruts, starruts };

// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="ruts"]
pub struct Rut {
    pub id: String,
    pub title: String,
    pub url: String,
    pub content: String,
    pub create_at: NaiveDateTime,
    pub renew_at: NaiveDateTime,
    pub author: String,  // todo, change as author
    pub uname: String,     // as who post
    pub credential: String,
    pub logo: String,
    pub item_count: i32,
    pub comment_count: i32,
    pub star_count: i32,
    pub vote: i32,       // cal per star, comment
    pub slug: String,
}

// Rut's constructor
impl Rut {
    pub fn new(uid: String, slug: String, rut: CreateRut) -> Self {
        Rut {
            id: uid,
            title: rut.title,
            url: rut.url,
            content: rut.content,
            create_at: Utc::now().naive_utc(),
            renew_at: Utc::now().naive_utc(),
            author: rut.author,
            uname: rut.uname,
            credential: rut.credential,
            logo: "".to_owned(),
            item_count: 0,
            comment_count: 0,
            star_count: 0,
            vote: 0,
            slug,
        }
    }
}

// as msg in create new
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct CreateRut {
    pub title: String,
    pub url: String,
    pub content: String,
    pub author: String,
    pub uname: String,
    pub credential: String,
}

impl Message for CreateRut {
    type Result = Result<RutMsg, ServiceError>;
}

impl Validate for CreateRut {
    fn validate(&self) -> Result<(), Error> {
        let url = &self.url.trim();
        let url_test = if url.len() == 0 { true } else { re_test_url(url) };
        let check_len =
            test_len_limit(&self.title, 3, TITLE_LEN) &&
            test_len_limit(&self.author, 0, 64) &&
            test_len_limit(&self.credential, 0, 64);
        let check = url_test && check_len;

        if check {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("Invalid Input"))
        }
    }
}

// as msg in select by id
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct QueryRut {
    pub rut_slug: String,
    // pub action: String, // get / delete, to do
}

impl Message for QueryRut {
    type Result = Result<RutMsg, ServiceError>;
}

// as msg to get  rut list, + paging
#[derive(Deserialize,Serialize,Debug, Clone)]
pub enum QueryRuts {
    Index(String),
    UserID(String, String, i32), // uname, create|star, paging
    ItemID(String, i32),
    TagID(String, i32),
    KeyID(String, String, String, i32), // keyword, per, perid(uname|item|tname), paging
}

impl Message for QueryRuts {
    type Result = Result<RutListMsg, ServiceError>;
}

// as msg in update rut
#[derive(Deserialize,Serialize,Debug,Clone,AsChangeset)]
#[table_name="ruts"]
pub struct UpdateRut {
    pub id: String,
    pub title: String,
    pub url: String,
    pub content: String,
    pub author: String,
    pub credential: String,
}

impl Message for UpdateRut {
    type Result = Result<RutMsg, ServiceError>;
}

impl Validate for UpdateRut {
    fn validate(&self) -> Result<(), Error> {
        let url = &self.url.trim();
        let url_test = if url.len() == 0 { true } else { re_test_url(url) };
        let check_len =
            test_len_limit(&self.title, 3, TITLE_LEN) &&
            test_len_limit(&self.author, 0, 64) &&
            test_len_limit(&self.credential, 0, 64);
        let check = url_test && check_len;

        if check {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("Invalid Input"))
        }
    }
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="starruts"]
pub struct StarRut {
    pub id: String,
    pub uname: String,
    pub rut_id: String,
    pub star_at: NaiveDateTime,
    pub note: String,
}

// as msg in star or unstar rut
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct StarOrRut {
    pub rut_id: String,
    pub uname: String,
    pub note: String,
    pub action: u8,  // 0- unstar, 1- star
}

impl Message for StarOrRut {
    type Result = Result<StarStatusMsg, ServiceError>;
}

// as msg to check if star a rut
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarRutStatus {
    pub uname: String,
    pub rut_id: String,
}

impl Message for StarRutStatus {
    type Result = Result<StarStatusMsg, ServiceError>;
}
