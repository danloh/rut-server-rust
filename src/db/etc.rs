// etc msg handler

use actix::{ Handler, Message };
use actix_web::{ FromRequest, HttpRequest, Error };
use diesel::prelude::*;
use diesel::{ self, QueryDsl, ExpressionMethods, dsl::any, PgTextExpressionMethods, RunQueryDsl };
use chrono::{ Local, NaiveDateTime, Utc, Duration };
use uuid::Uuid;

use crate::Dba;
use crate::errors::ServiceError;
use crate::db::msg::{ Msg, EtcMsg, EtcListMsg };
use crate::PER_PAGE;
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

// handle msg from api::etc.post_etc
impl Handler<PostEtc> for Dba {
    type Result = Result<EtcMsg, ServiceError>;

    fn handle(&mut self, new_e: PostEtc, _: &mut Self::Context) -> Self::Result {

        use crate::schema::etcs::dsl::*;
        let conn = &self.0.get().unwrap();
        
        // extract the id
        use std::collections::HashMap;
        use crate::util::share::get_v;
        let mut id_map = HashMap::new();
        id_map.insert(new_e.post_to.clone(), new_e.to_id.clone());
        
        let uid = format!("{}", uuid::Uuid::new_v4());
        let newetc = Etc {
            id: uid,
            content: new_e.content,
            post_at: Utc::now().naive_utc(),
            petc_id: get_v(&id_map, "petc"),
            rut_id: get_v(&id_map, "rut"),
            item_id: get_v(&id_map, "item"),
            tname: get_v(&id_map, "tag"),
            uname: new_e.uname,
            vote: 1,
        };
        let etc_new = diesel::insert_into(etcs)
            .values(&newetc).get_result::<Etc>(conn)?;
        
        if &new_e.post_to == "rut" {
            use crate::schema::ruts::dsl::*;
            diesel::update(ruts.filter(&id.eq(&new_e.to_id)))
                .set(comment_count.eq(comment_count + 1)).execute(conn)?;
        }

        Ok( EtcMsg { 
            status: 201, 
            message: "Posted".to_string(),
            etc: etc_new.clone(),
        })
    }
}

// as msg to get etc list
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EtcsPerID {     // diff way from enum to get
    pub per: String,
    pub perid: String,
    pub page: i32,
}

impl Message for EtcsPerID {
    type Result = Result<EtcListMsg, ServiceError>;
}

// handle msg from api::etc.get_etc_list
impl Handler<EtcsPerID> for Dba {
    type Result = Result<EtcListMsg, ServiceError>;

    fn handle(&mut self, per: EtcsPerID, _: &mut Self::Context) -> Self::Result {
        use crate::schema::etcs::dsl::*;
        let conn = &self.0.get().unwrap();
        
        let p = per.page;
        // eliminate no limit
        if p < 1 {
            return Ok( EtcListMsg { 
                status: 400, 
                message: "Nothing".to_string(),
                etcs: Vec::new(),
                count: 0,
            })
        }

        let per_id = &per.perid;
        let per_to = per.per.trim();

        let etc_list = match per_to {
            "rut" => {
                etcs.filter(&rut_id.eq(per_id))
                    .order(post_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Etc>(conn)?
            },
            "item" => {
                etcs.filter(&item_id.eq(per_id)).order(post_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Etc>(conn)?
            },
            "tag" => {
                etcs.filter(&tname.eq(per_id)).order(post_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Etc>(conn)?
            },
            "petc" => {
                etcs.filter(&petc_id.eq(per_id)).order(post_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Etc>(conn)?
            },
            "user" => {
                etcs.filter(&uname.eq(per_id)).order(post_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Etc>(conn)?
            },
            _ => { // just get some newest
                etcs.order(post_at.desc())
                    .limit(PER_PAGE.into()).load::<Etc>(conn)?
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
