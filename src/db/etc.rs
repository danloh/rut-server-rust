// etc msg handler

use db::dba::Dba;
use actix_web::{ actix::Handler, error, Error };
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl };
use chrono::Utc;
use uuid;

use model::etc::{ Etc, NewEtc, PostEtc, DelEtc, EtcsPerID };
use model::msg::{ Msg, EtcMsg, EtcListMsg };
use PER_PAGE;

/// etc models: excerpt, article, review, comment, etc.

use crate::schema::etcs;
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::{ Msg, EtcMsg, EtcListMsg };

// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
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

// use to build insert query
#[derive(Debug,Clone,Serialize,Deserialize,Insertable)]
#[table_name="etcs"]
pub struct NewEtc<'a> {
    pub id: &'a str,
    pub content: &'a str,
    pub post_at: NaiveDateTime,
    pub petc_id: &'a str,
    pub rut_id: &'a str,
    pub item_id: &'a str,
    pub tname: &'a str,
    pub uname: &'a str,
    pub vote: i32,
}

// as msg in create new
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostEtc {
    pub content: String,
    pub post_to: String,
    pub to_id: String,
    pub uname: String,
}

impl Message for PostEtc {
    type Result = Result<EtcMsg, Error>;
}

// as msg to get etc list
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EtcsPerID {     // diff way from enum to get
    pub per: String,
    pub per_id: String,
    pub paging: i32,
}

impl Message for EtcsPerID {
    type Result = Result<EtcListMsg, Error>;
}

// as msg to del etc
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DelEtc {
    pub etc_id: String,
    pub rut_id: String,   // to update rut after del
    pub item_id: String,  // to update item after del
    pub uname: String,  // to check permission
}

impl Message for DelEtc {
    type Result = Result<Msg, Error>;
}


// handle msg from api::etc.post_etc
impl Handler<PostEtc> for Dba {
    type Result = Result<EtcMsg, Error>;

    fn handle(&mut self, new_e: PostEtc, _: &mut Self::Context) -> Self::Result {

        use db::schema::etcs::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        // extract the id
        use std::collections::HashMap;
        use util::share::get_v;
        let mut id_map = HashMap::new();
        id_map.insert(new_e.post_to.clone(), new_e.to_id.clone());
        
        let uid = format!("{}", uuid::Uuid::new_v4());
        let newetc = NewEtc {
            id: &uid,
            content: &new_e.content,
            post_at: Utc::now().naive_utc(),
            petc_id: &get_v(&id_map, "petc"),
            rut_id: &get_v(&id_map, "rut"),
            item_id: &get_v(&id_map, "item"),
            tname: &get_v(&id_map, "tag"),
            uname: &new_e.uname,
            vote: 1,
        };
        let etc_new = diesel::insert_into(etcs)
            .values(&newetc)
            .get_result::<Etc>(conn)
            .map_err(error::ErrorInternalServerError)?;
        
        if &new_e.post_to == "rut" {
            use db::schema::ruts::dsl::*;
            diesel::update(ruts.filter(&id.eq(&new_e.to_id)))
                .set(comment_count.eq(comment_count + 1)).execute(conn)
                .map_err(error::ErrorInternalServerError)?;
        }

        Ok( EtcMsg { 
            status: 201, 
            message: "Posted".to_string(),
            etc: etc_new.clone(),
        })
    }
}

// handle msg from api::etc.get_etc_list
impl Handler<EtcsPerID> for Dba {
    type Result = Result<EtcListMsg, Error>;

    fn handle(&mut self, per: EtcsPerID, _: &mut Self::Context) -> Self::Result {
        use db::schema::etcs::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let p = per.paging;
        // eliminate no limit
        if p < 1 {
            return Ok( EtcListMsg { 
                status: 400, 
                message: "Nothing".to_string(),
                etcs: Vec::new(),
                count: 0,
            })
        }

        let per_id = &per.per_id;
        let per_to = per.per.trim();

        let etc_list = match per_to {
            "rut" => {
                etcs.filter(&rut_id.eq(per_id))
                    .order(post_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Etc>(conn).map_err(error::ErrorInternalServerError)?
            },
            "item" => {
                etcs.filter(&item_id.eq(per_id)).order(post_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Etc>(conn).map_err(error::ErrorInternalServerError)?
            },
            "tag" => {
                etcs.filter(&tname.eq(per_id)).order(post_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Etc>(conn).map_err(error::ErrorInternalServerError)?
            },
            "petc" => {
                etcs.filter(&petc_id.eq(per_id)).order(post_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Etc>(conn).map_err(error::ErrorInternalServerError)?
            },
            "user" => {
                etcs.filter(&uname.eq(per_id)).order(post_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Etc>(conn).map_err(error::ErrorInternalServerError)?
            },
            _ => { // just get some newest
                etcs.order(post_at.desc())
                    .limit(PER_PAGE.into()).load::<Etc>(conn)
                    .map_err(error::ErrorInternalServerError)?
            },

        };
        
        Ok( EtcListMsg { 
            status: 200, 
            message: "Get".to_string(),
            etcs: etc_list.clone(),
            count: etc_list.len(),
        })
    }
}
