// tag typed model and msg handler

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
use crate::model::msg::{ Msg, TagMsg, TagListMsg, StarStatusMsg };
use crate::model::tag::{ 
    Tag, CheckTag, UpdateTag, QueryTags, TagRut, RutTag, 
    TagItem, TagEtc, TagAny, StarTag, StarOrTag, StarTagStatus 
};

// handle msg from api::tag.new_tag and get_tag
impl Handler<CheckTag> for Dba {
    type Result = Result<TagMsg, ServiceError>;

    fn handle(&mut self, tg: CheckTag, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tags::dsl::*;
        let conn = &self.0.get()?;
        
        let action = tg.action.trim();
        if action == "POST" {
            let newtag = Tag::new(tg.tname);
            let tag_new = diesel::insert_into(tags)
                .values(&newtag).get_result::<Tag>(conn)?;

            Ok( TagMsg { 
                status: 200, 
                message: "Added".to_string(),
                tag: tag_new,
            })
        } else { // GET
            let tag_q = tags.filter(&tname.eq(&tg.tname)).get_result::<Tag>(conn)?;

            Ok( TagMsg { 
                status: 200, 
                message: "Get".to_string(),
                tag: tag_q,
            })
        }
    }
}

// handle msg from api::tag.get_tag_list
impl Handler<QueryTags> for Dba {
    type Result = Result<TagListMsg, ServiceError>;

    fn handle(&mut self, per: QueryTags, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tags::dsl::*;
        let conn = &self.0.get()?;
        
        let mut tag_list: Vec<String> = Vec::new();
        
        match per {
            QueryTags::RutID(r) => {
                use crate::schema::tagruts::dsl::*;
                tag_list = tagruts.filter(&rut_id.eq(&r)).select(tname)
                    .order(count.desc()).limit(10) // order per count
                    .load::<String>(conn)?;
            },
            QueryTags::ItemID(i) => {
                use crate::schema::tagitems::dsl::*;
                tag_list = tagitems.filter(&item_id.eq(&i)).select(tname)
                    .order(count.desc()).limit(10) 
                    .load::<String>(conn)?;
            },
            QueryTags::TagID(t) => {
                tag_list = tags.filter(&pname.eq(&t)).select(tname)
                    .order(vote.desc()).limit(10) 
                    .load::<String>(conn)?;
            },
            QueryTags::UserID(u) => { 
                use crate::schema::startags::dsl::*;
                tag_list = startags.filter(&uname.eq(&u)).select(tname)
                    .order(star_at.desc()).limit(42) 
                    .load::<String>(conn)?;
            },
            QueryTags::Index(_) => {
                tag_list = tags.select(tname).order(vote.desc()).limit(16)
                    .load::<String>(conn)?;
            },
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
    type Result = Result<TagMsg, ServiceError>;

    fn handle(&mut self, tg: UpdateTag, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tags::dsl::*;
        let conn = &self.0.get()?;

        let tag_update = diesel::update(tags.filter(&tname.eq(&tg.tname)))
            .set((
                intro.eq(tg.intro),
                logo.eq(tg.logo),
                pname.eq(tg.pname),  // to check if pname existing?
            ))
            .get_result::<Tag>(conn)?;

        Ok( TagMsg { 
            status: 201, 
            message: "Updated".to_string(),
            tag: tag_update,
        })
    }
}

// handle msg from api::tag.tag_rut :  to be optimized
impl Handler<RutTag> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, rutg: RutTag, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tagruts::dsl::*;
        let conn = &self.0.get()?;

        let action = rutg.action;
        let rutID = rutg.rut_id;

        if action == 1 {  // tag
            for rtg in rutg.tnames {
                // to check if tagged with a same tag
                let tr = tagruts.filter(&tname.eq(&rtg)).filter(&rut_id.eq(&rutID))
                    .load::<TagRut>(conn)?.pop();
                match tr {
                    // if tagged, update count + 1 in tagruts
                    Some(tgr) => {
                        diesel::update(&tgr).set(count.eq(count + 1)).execute(conn)?;
                    },
                    // else new tag-rut
                    None => {
                        let new_tag_rut = TagRut {
                            id: (rtg.clone() + "-" + &rutID),
                            tname: rtg.clone(),
                            rut_id: rutID.clone(),
                            count: 1,
                        };
                        //  to check if tname in tags? otherwise, new_tag
                        use crate::schema::tags::dsl::*;
                        let tag_check = tags.filter(&tname.eq(&rtg)).load::<Tag>(conn)?.pop();
                        match tag_check {
                            // if existing, tag then rut_count + 1 in tags
                            Some(t) => {
                                diesel::insert_into(tagruts).values(&new_tag_rut).execute(conn)?;
                                // then update tags.rut_count
                                diesel::update(&t).set(rut_count.eq(rut_count + 1)).execute(conn)?;
                            },
                            // if no existing tname, new_tag
                            None => {
                                let newtag = Tag {
                                    id: rtg.clone(),
                                    tname: rtg.clone(),
                                    intro: "".to_owned(),
                                    logo: "".to_owned(),
                                    pname: "".to_owned(),
                                    item_count: 0,
                                    rut_count: 1,
                                    etc_count: 0,
                                    star_count: 0,
                                    vote: 0,
                                };
                                // new_tag 
                                diesel::insert_into(tags).values(&newtag).execute(conn)?;
                                // then tag_rut
                                diesel::insert_into(tagruts).values(&new_tag_rut).execute(conn)?;
                            },
                        }  
                    },
                }
            }
        } else { // untag
            for rtg in rutg.tnames {
                diesel::delete(tagruts.filter(&tname.eq(&rtg))).execute(conn)?;
            }
        }

        Ok( Msg { 
            status: 201, 
            message: "Done".to_string(),
        })
    }
}

// handle msg from api::tag.star_unstar_tag
impl Handler<StarOrTag> for Dba {
    type Result = Result<StarStatusMsg, ServiceError>;

    fn handle(&mut self, tstar: StarOrTag, _: &mut Self::Context) -> Self::Result {
        use crate::schema::startags::dsl::*;
        use crate::schema::tags::dsl::{tags, tname as t_name, star_count, rut_count, vote};
        let conn = &self.0.get()?;

        let tag_query = tags.filter(&t_name.eq(&tstar.tname))
            .get_result::<Tag>(conn)?;
        let s_count = tag_query.star_count;
        
        match tstar.action {
            1  => {  // star
                // limit user to star tag to 42
                let tag_star_num = 
                    startags.filter(&uname.eq(&tstar.uname)).count().execute(conn)?;
                if tag_star_num > 42 {
                    return Ok( StarStatusMsg{ status: 418, message: "unstar".to_string(), count: 42,})
                }
                
                let uid = format!("{}", uuid::Uuid::new_v4());
                let new_star = StarTag {
                    id: uid,
                    uname: tstar.clone().uname,
                    tname: tstar.clone().tname,
                    star_at: Utc::now().naive_utc(),
                    note: tstar.clone().note,
                };
                diesel::insert_into(startags).values(&new_star).execute(conn)?;
                // to update star_count + 1 in tag
                diesel::update(&tag_query)
                    .set((
                        star_count.eq(star_count + 1),
                        vote.eq(rut_count * 2 + star_count)  // cal vote, to be task
                    ))
                    .execute(conn)?;

                Ok( StarStatusMsg{ status: 200, message: "star".to_string(), count: s_count+1,})
            },
            0 => { // unsatr
                diesel::delete(
                    startags.filter(&tname.eq(&tstar.tname))
                            .filter(&uname.eq(&tstar.uname))
                )
                .execute(conn)?;

                // to update the star_count - 1 in tag
                diesel::update(&tag_query)
                    .set(star_count.eq(star_count - 1)).execute(conn)?;

                Ok( StarStatusMsg{ status: 200, message: "unstar".to_string(), count: s_count-1,})
            },
            _ =>  { Ok( StarStatusMsg{ status: 400, message: "unstar".to_string(), count: s_count,}) },
        }
    }
}

// handle msg from api::tag.star_tag_status
impl Handler<StarTagStatus> for Dba {
    type Result = Result<StarStatusMsg, ServiceError>;

    fn handle(&mut self, status: StarTagStatus, _: &mut Self::Context) -> Self::Result {
        use crate::schema::startags::dsl::*;
        use crate::schema::tags::dsl::{tags, tname as t_name, star_count};
        let conn = &self.0.get()?;

        let check_status = startags
            .filter(&uname.eq(&status.uname))
            .filter(&tname.eq(&status.tname))
            .load::<StarTag>(conn)?.pop();
            
        let s_count = tags.filter(&t_name.eq(&status.tname))
            .select(star_count).get_result::<i32>(conn)?;

        let msg = match check_status {
            Some(_) => { "star" },
            None => { "unstar" },
        };
        Ok( StarStatusMsg{ status: 200, message: msg.to_string(), count: s_count}) 
    }
}

// handle tag rut|item|etc
impl Handler<TagAny> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, tg: TagAny, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        
        let tgnames = tg.tnames;
        let tag_to = tg.tag_to.trim();
        let action = tg.action;
        let toID = tg.to_id;

        match tag_to {
            "rut" => {
                use crate::schema::tagruts::dsl::*;
                for rtg in tgnames {
                    if action == 1 {  // tag
                        // to check if tagged with a same tag
                        let tr = tagruts.filter(&tname.eq(&rtg)).filter(&rut_id.eq(&toID))
                            .load::<TagRut>(conn)?.pop();
                        match tr {
                            // if tagged, update count + 1 in tagruts
                            Some(tgr) => {
                                diesel::update(&tgr).set(count.eq(count + 1)).execute(conn)?;
                            },
                            // else new tag-rut
                            None => {
                                let new_tag_rut = TagRut {
                                    id: (rtg.clone() + "-" + &toID),
                                    tname: rtg.clone(),
                                    rut_id: toID.clone(),
                                    count: 1,
                                };
                                diesel::insert_into(tagruts).values(&new_tag_rut).execute(conn)?;
                                // check tnames if existing
                                use crate::schema::tags::dsl::{tags, tname as t_name, rut_count};
                                let tag_check = tags.filter(&t_name.eq(&rtg)).load::<Tag>(conn)?.pop();
                                match tag_check {
                                    Some(t) => {
                                        // then update tags.rut_count
                                        diesel::update(&t).set(rut_count.eq(rut_count + 1)).execute(conn)?;
                                    },
                                    None => {
                                        let newtag = Tag{
                                            rut_count: 1,
                                            ..Tag::new(rtg)
                                        };
                                        // new_tag 
                                        diesel::insert_into(tags).values(&newtag).execute(conn)?;
                                    }
                                }
                            },
                        }
                    } else { // untag
                        diesel::delete(tagruts.filter(&tname.eq(&rtg))).execute(conn)?;
                    }
                }
            },
            "item" => {
                use crate::schema::tagitems::dsl::*;
                for itg in tgnames {
                    if action == 1 {  // tag
                        // to check if tagged with a same tag
                        let ti = tagitems.filter(&tname.eq(&itg)).filter(&item_id.eq(&toID))
                            .load::<TagItem>(conn)?.pop();
                        match ti {
                            // if tagged, update count + 1
                            Some(tgi) => {
                                diesel::update(&tgi).set(count.eq(count + 1)).execute(conn)?;
                            },
                            // else new tag-rut
                            None => {
                                let new_tag_item = TagItem {
                                    id: itg.clone() + "-" + &toID,
                                    tname: itg.clone(),
                                    item_id: toID.clone(),
                                    count: 1,
                                };
                                diesel::insert_into(tagitems).values(&new_tag_item).execute(conn)?; 
                                // check tnames if existing
                                use crate::schema::tags::dsl::{tags, tname as t_name, item_count};
                                let tag_check = tags.filter(&t_name.eq(&itg)).load::<Tag>(conn)?.pop();
                                match tag_check {
                                    Some(t) => {
                                        // then update tags.rut_count
                                        diesel::update(&t).set(item_count.eq(item_count + 1)).execute(conn)?;
                                    },
                                    None => {
                                        let newtag = Tag{
                                            item_count: 1,
                                            ..Tag::new(itg)
                                        };
                                        // new_tag 
                                        diesel::insert_into(tags).values(&newtag).execute(conn)?;
                                    }
                                } 
                            },
                        }
                    } else { // untag
                        diesel::delete(tagitems.filter(&tname.eq(&itg))).execute(conn)?;
                    }
                }
            },
            "etc" => {
                use crate::schema::tagetcs::dsl::*;
                for etg in tgnames {
                    // to check if tagged with a same tag
                    let te = tagetcs.filter(&tname.eq(&etg)).filter(&etc_id.eq(&toID))
                        .load::<TagEtc>(conn)?.pop();
                    if let None = te {
                        let new_tag_etc = TagEtc {
                            id: etg.clone() + "-" + &toID,
                            tname: etg.clone(),
                            etc_id: toID.clone(),
                        };
                        diesel::insert_into(tagetcs).values(&new_tag_etc).execute(conn)?; 
                        // check tnames if existing
                        use crate::schema::tags::dsl::{tags, tname as t_name};
                        let tag_check = tags.filter(&t_name.eq(&etg)).load::<Tag>(conn)?.pop();
                        if let None = tag_check {
                            let newtag = Tag{ etc_count: 1, ..Tag::new(etg) };
                            // new_tag 
                            diesel::insert_into(tags).values(&newtag).execute(conn)?;
                        }
                    }
                }
            },
            _  => (),
        }

        Ok( Msg { 
            status: 201, 
            message: "Done".to_string(),
        })
    }
}
