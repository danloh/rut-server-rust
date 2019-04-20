// etc typed model and msg handler

use actix::{ Handler };
use diesel::prelude::*;
use diesel::{ 
    self, QueryDsl, ExpressionMethods, 
    dsl::any, PgTextExpressionMethods, RunQueryDsl 
};
use chrono::{ Utc };
use uuid::Uuid;

use crate::Dba;
use crate::errors::ServiceError;
use crate::model::msg::{ Msg, EtcMsg, EtcListMsg };
use crate::PER_PAGE;
use crate::model::etc::{ Etc, PostEtc, QueryEtcs };

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

// handle msg from api::etc.get_etc_list
impl Handler<QueryEtcs> for Dba {
    type Result = Result<EtcListMsg, ServiceError>;

    fn handle(&mut self, per: QueryEtcs, _: &mut Self::Context) -> Self::Result {
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
