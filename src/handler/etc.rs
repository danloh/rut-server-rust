// etc msg handler

use db::dba::Dba;
use actix_web::{ actix::Handler, error, Error };
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl };
use chrono::Utc;
use uuid;

use model::etc::{ Etc, NewEtc, PostEtc, DelEtc, EtcsPerID };
use model::msg::{ Msg, EtcMsg, EtcListMsg };

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
        
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let newetc = NewEtc {
            id: &uuid,
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
            status: 200, 
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
        
        let per_id = &per.per_id;
        let per_to = &per.per;
        let etc_list = if per_to == "rut" {
            etcs.filter(&rut_id.eq(per_id)).load::<Etc>(conn).map_err(error::ErrorInternalServerError)?
        } else if per_to == "item" {
            etcs.filter(&item_id.eq(per_id)).load::<Etc>(conn).map_err(error::ErrorInternalServerError)?
        } else if per_to == "tag" {
            etcs.filter(&tname.eq(per_id)).load::<Etc>(conn).map_err(error::ErrorInternalServerError)?
        } else if per_to == "petc" {
            etcs.filter(&petc_id.eq(per_id)).load::<Etc>(conn).map_err(error::ErrorInternalServerError)?
        } else {
            etcs.filter(&uname.eq(per_id)).load::<Etc>(conn).map_err(error::ErrorInternalServerError)?
        };  // why cannot match?
        
        Ok( EtcListMsg { 
            status: 200, 
            message: "Get".to_string(),
            etcs: etc_list.clone(),
            count: etc_list.len(),
        })
    }
}
