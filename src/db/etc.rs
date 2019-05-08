// etc typed model and msg handler

use actix::Handler;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{
    self, dsl::any, ExpressionMethods, 
    PgTextExpressionMethods, QueryDsl, RunQueryDsl
};
use uuid::Uuid;

use crate::errors::ServiceError;
use crate::model::etc::{Etc, PostEtc, QueryEtcs};
use crate::model::msg::{EtcListMsg, EtcMsg, Msg};
use crate::model::PER_PAGE;
use crate::Dba;

// handle msg from api::etc.post_etc
impl Handler<PostEtc> for Dba {
    type Result = Result<EtcMsg, ServiceError>;

    fn handle(&mut self, new_etc: PostEtc, _: &mut Self::Context) -> Self::Result {
        use crate::schema::etcs::dsl::*;
        let conn = &self.0.get()?;

        // extract the id
        use crate::util::share::get_v;
        use std::collections::HashMap;
        let mut id_map = HashMap::new();
        id_map.insert(new_etc.post_to.clone(), new_etc.to_id.clone());

        let uid = format!("{}", uuid::Uuid::new_v4());
        let newetc = Etc {
            id: uid,
            content: new_etc.content,
            post_at: Utc::now().naive_utc(),
            petc_id: get_v(&id_map, "petc"),
            rut_id: get_v(&id_map, "rut"),
            item_id: get_v(&id_map, "item"),
            tname: get_v(&id_map, "tag"),
            uname: new_etc.uname,
            vote: 1,
        };
        let etc_new = diesel::insert_into(etcs)
            .values(&newetc)
            .get_result::<Etc>(conn)?;

        // update comment_count + 1 in ruts
        if &new_etc.post_to == "rut" {
            use crate::schema::ruts::dsl::*;
            diesel::update(ruts.filter(&id.eq(&new_etc.to_id)))
                .set(comment_count.eq(comment_count + 1))
                .execute(conn)?;
        }

        Ok(EtcMsg {
            status: 201,
            message: "Posted".to_string(),
            etc: etc_new,
        })
    }
}

// handle msg from api::etc.get_etc_list
impl Handler<QueryEtcs> for Dba {
    type Result = Result<EtcListMsg, ServiceError>;

    fn handle(&mut self, per: QueryEtcs, _: &mut Self::Context) -> Self::Result {
        use crate::schema::etcs::dsl::*;
        let conn = &self.0.get()?;

        let p = per.page;
        // eliminate no limit
        if p < 1 {
            return Err(ServiceError::BadRequest(
                "400: No Requested Resource".into(),
            ));
        }

        let per_id = &per.perid;
        let per_to = per.per.trim();

        let etc_list = match per_to {
            "rut" => etcs
                .filter(&rut_id.eq(per_id))
                .order(post_at.desc())
                .limit(PER_PAGE.into())
                .offset((PER_PAGE * (p - 1)).into())
                .load::<Etc>(conn)?,
            "item" => etcs
                .filter(&item_id.eq(per_id))
                .order(post_at.desc())
                .limit(PER_PAGE.into())
                .offset((PER_PAGE * (p - 1)).into())
                .load::<Etc>(conn)?,
            "tag" => etcs
                .filter(&tname.eq(per_id))
                .order(post_at.desc())
                .limit(PER_PAGE.into())
                .offset((PER_PAGE * (p - 1)).into())
                .load::<Etc>(conn)?,
            "petc" => etcs
                .filter(&petc_id.eq(per_id))
                .order(post_at.desc())
                .limit(PER_PAGE.into())
                .offset((PER_PAGE * (p - 1)).into())
                .load::<Etc>(conn)?,
            "user" => etcs
                .filter(&uname.eq(per_id))
                .order(post_at.desc())
                .limit(PER_PAGE.into())
                .offset((PER_PAGE * (p - 1)).into())
                .load::<Etc>(conn)?,
            _ => {
                // just get some newest
                etcs.order(post_at.desc())
                    .limit(PER_PAGE.into())
                    .load::<Etc>(conn)?
            }
        };

        Ok(EtcListMsg {
            status: 200,
            message: "Get".to_string(),
            etcs: etc_list.clone(),
            count: etc_list.len(),
        })
    }
}
