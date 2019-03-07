// tag msg handler

use db::dba::Dba;
use actix_web::{ actix::Handler, error, Error };
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl };
use chrono::Utc;
use uuid;

use model::tag::{ 
    Tag, NewTag, CheckTag, UpdateTag, TagsPerID, TagRut, NewTagRut, RutTag 
};
use model::msg::{ Msg, TagMsg, TagListMsg };

// handle msg from api::tag.new_tag and get_tag
impl Handler<CheckTag> for Dba {
    type Result = Result<TagMsg, Error>;

    fn handle(&mut self, tg: CheckTag, _: &mut Self::Context) -> Self::Result {
        use db::schema::tags::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let action = tg.action;
        if &action == "POST" {
            let newtag = NewTag {
                id: &tg.tname,
                tname: &tg.tname,
                intro: "",
                logo: "",
                pname: "",
                item_count: 0,
                rut_count: 0,
                etc_count: 0,
                star_count: 0,
            };
            let tag_new = diesel::insert_into(tags)
                .values(&newtag)
                .get_result::<Tag>(conn)
                .map_err(error::ErrorInternalServerError)?;

            Ok( TagMsg { 
                status: 200, 
                message: "Added".to_string(),
                tag: tag_new.clone(),
            })
        } else { // GET
            let tag_q = tags.filter(&tname.eq(&tg.tname))
                .get_result::<Tag>(conn)
                .map_err(error::ErrorInternalServerError)?;

            Ok( TagMsg { 
                status: 200, 
                message: "Get".to_string(),
                tag: tag_q.clone(),
            })
        }
    }
}

// handle msg from api::tag.get_tag_list
impl Handler<TagsPerID> for Dba {
    type Result = Result<TagListMsg, Error>;

    fn handle(&mut self, per: TagsPerID, _: &mut Self::Context) -> Self::Result {
        use db::schema::tags::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let mut tag_list: Vec<String> = Vec::new();
        
        match per {
            TagsPerID::RutID(r) => {
                use db::schema::tagruts::dsl::*;
                tag_list = tagruts.filter(&rut_id.eq(&r)).select(tname)
                    .order(count.desc()).limit(10) // order per count
                    .load::<String>(conn)           
                    .map_err(error::ErrorInternalServerError)?;
            },
            TagsPerID::ItemID(i) => {
                use db::schema::tagitems::dsl::*;
                tag_list = tagitems.filter(&item_id.eq(&i)).select(tname)
                    .order(count.desc()).limit(10) 
                    .load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            TagsPerID::TagID(t) => {
                tag_list = tags.filter(&pname.eq(&t)).select(tname)
                    .order(vote.desc()).limit(10) 
                    .load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            TagsPerID::UserID(_u) => { tag_list = Vec::new(); }, // to do, limit to 42
        }

        Ok( TagListMsg { 
            status: 200, 
            message: "Success".to_string(),
            tags: tag_list.clone(),
            count: tag_list.len(),
        })
    }
}

// handle msg from api::tag.update_tag
impl Handler<UpdateTag> for Dba {
    type Result = Result<TagMsg, Error>;

    fn handle(&mut self, tg: UpdateTag, _: &mut Self::Context) -> Self::Result {
        use db::schema::tags::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let tag_update = diesel::update(tags.filter(&tname.eq(&tg.tname)))
            .set( &UpdateTag{
                tname: tg.tname.clone(),
                intro: tg.intro.clone(),
                logo: tg.logo.clone(),
                pname: tg.pname.clone(), 
            })
            .get_result::<Tag>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok( TagMsg { 
            status: 200, 
            message: "Updated".to_string(),
            tag: tag_update.clone(),
        })
    }
}

// handle msg from api::tag.tag_rut :  to be optimized
impl Handler<RutTag> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, rtgs: RutTag, _: &mut Self::Context) -> Self::Result {
        use db::schema::tagruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let action = rtgs.action;

        if &action == "1" {  // tag
            for rtg in rtgs.tname {
                let tr = tagruts.filter(&tname.eq(&rtg)).load::<TagRut>(conn)
                    .map_err(error::ErrorInternalServerError)?.pop();
                match tr {
                    Some(tgr) => {
                        diesel::update(&tgr)
                        .set(count.eq(count + 1))
                        .execute(conn).map_err(error::ErrorInternalServerError)?;
                    },
                    None => {
                        let new_tag_rut = NewTagRut {
                            id: &rtg,
                            tname: &rtg,
                            rut_id: &rtgs.rut_id,
                            count: 1,
                        };
                        //  to check if tname in tags? otherwise, new_tag
                        use db::schema::tags::dsl::*;
                        let tag_check = tags.filter(&tname.eq(&rtg)).load::<Tag>(conn)
                            .map_err(error::ErrorInternalServerError)?.pop();
                        match tag_check {
                            // if existing, rut_count + 1
                            Some(t) => {
                                diesel::insert_into(tagruts).values(&new_tag_rut)
                                    .execute(conn).map_err(error::ErrorInternalServerError)?;
                                // then update tags.rut_count
                                diesel::update(&t)
                                .set(rut_count.eq(rut_count + 1)).execute(conn)
                                .map_err(error::ErrorInternalServerError)?;
                            },
                            // if no existing, new_tag
                            None => {
                                let newtag = NewTag {
                                    id: &rtg,
                                    tname: &rtg,
                                    intro: "",
                                    logo: "",
                                    pname: "",
                                    item_count: 0,
                                    rut_count: 1,
                                    etc_count: 0,
                                    star_count: 0,
                                };
                                // new_tag then tag_rut
                                diesel::insert_into(tags).values(&newtag).execute(conn)
                                    .map_err(error::ErrorInternalServerError)?;
                                diesel::insert_into(tagruts).values(&new_tag_rut)
                                    .execute(conn).map_err(error::ErrorInternalServerError)?;
                            },
                        }  
                    },
                }
            }
        } else { // untag
            for rtg in rtgs.tname {
                diesel::delete(tagruts.filter(&tname.eq(&rtg))) 
                .execute(conn).map_err(error::ErrorInternalServerError)?;
            }
        }

        Ok( Msg { 
            status: 200, 
            message: "Done".to_string(),
        })
    }
}
