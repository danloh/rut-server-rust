// item typed model and msg handler

use actix::{ Handler };
use diesel::prelude::*;
use diesel::{ 
    self, QueryDsl, ExpressionMethods, 
    dsl::any, PgTextExpressionMethods, RunQueryDsl 
};
use chrono::{ Utc };
use uuid::Uuid;

use crate::Dba;
use crate::{ PER_PAGE, ANS_LIMIT };
use crate::errors::ServiceError;
use crate::util::share::{ gen_slug };
use crate::model::msg::{ Msg, ItemMsg, ItemListMsg, StarItemMsg, CollectMsg, CollectsMsg };
use crate::model::rut::Rut;
use crate::model::item::{ 
   Item, NewItem, UpdateItem, QueryItem, QueryItems, CollectItem, 
   Collect, QueryCollects, QueryCollect, UpdateCollect, DelCollect, 
   StarItem, NewStarItem, StarItemStatus
};


// handle msg from api::item.submit_item
impl Handler<NewItem> for Dba {
    type Result = Result<ItemMsg, ServiceError>;

    fn handle(&mut self, submit: NewItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items::dsl::*;
        let conn = &self.0.get().unwrap();
        
        // check if existing, field may be ""
        // do not use or_filter()
        let s_uiid = &submit.uiid;
        let s_url = &submit.url;
        if s_uiid.trim() != "" {
            let check_q_id = items.filter(&uiid.eq(s_uiid))
                .load::<Item>(conn)?.pop();
            if let Some(i) = check_q_id {
                return Ok( ItemMsg { 
                    status: 422, 
                    message: "Existing".to_string(),
                    item: i,
                })
            }
        }
        if s_url.trim() != "" {
            let check_q_id = items.filter(&url.eq(s_url))
                .load::<Item>(conn)?.pop();
            if let Some(i) = check_q_id {
                return Ok( ItemMsg { 
                    status: 422, 
                    message: "Existing".to_string(),
                    item: i,
                })
            }
        }
        
        let uuid_v4 = uuid::Uuid::new_v4();
        let uid = format!("{}", uuid_v4);
        let i_slug = gen_slug("i", &submit.title, &uuid_v4);
        let new_item = Item::new(uid, i_slug, submit);
        let item_new = diesel::insert_into(items)
            .values(&new_item)
            .get_result::<Item>(conn)?;

        Ok( ItemMsg { 
            status: 201, 
            message: "Submitted".to_string(),
            item: item_new.clone(),
        })
    }
}


// handle msg from api::item.update_item
impl Handler<UpdateItem> for Dba {
    type Result = Result<ItemMsg, ServiceError>;

    fn handle(&mut self, item: UpdateItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items::dsl::*;
        let conn = &self.0.get().unwrap();

        let item_update = diesel::update(items.filter(&id.eq(&item.id)))
            .set(&item)
            .get_result::<Item>(conn)?;

        Ok( ItemMsg { 
            status: 201,
            message: "Updated".to_string(),
            item: item_update.clone(),
        })
    }
}

// handle msg from api::item.get_item
impl Handler<QueryItem> for Dba {
    type Result = Result<ItemMsg, ServiceError>;

    fn handle(&mut self, islug: QueryItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items::dsl::*;
        let conn = &self.0.get().unwrap();

        let item_query = items.filter(&slug.eq(&islug.item_slug)) // slug here only
            .get_result::<Item>(conn)?;

        Ok( ItemMsg { 
            status: 200, 
            message: "Success".to_string(),
            item: item_query,
        })
    }
}

// handle msg from api::item.get_item_list
impl Handler<QueryItems> for Dba {
    type Result = Result<ItemListMsg, ServiceError>;

    fn handle(&mut self, perid: QueryItems, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items::dsl::*;
        let conn = &self.0.get().unwrap();
        
        let mut item_id_vec: Vec<String> = Vec::new();
        let mut item_list: Vec<Item> = Vec::new();
        let mut item_num = 0;  // total
        
        // better do some limit
        match perid {
            QueryItems::ItemID(i) => {
                item_list = items.filter(&id.eq(&i)).load::<Item>(conn)?;
            },
            QueryItems::Title(t) => {
                item_list = items
                    .filter(&title.ilike(&t)) // ilike: %k%, %k, k%
                    .or_filter(&uiid.ilike(&t)).limit(10)
                    .load::<Item>(conn)?;
            },
            QueryItems::Uiid(d) => {
                item_list = items
                    .filter(&uiid.ilike(&d))
                    .or_filter(&title.ilike(&d)).limit(10)
                    .load::<Item>(conn)?;
            },
            QueryItems::ItemUrl(u) => {
                item_list = items
                    .filter(&url.ilike(&u)).limit(10)
                    .load::<Item>(conn)?;
            },
            QueryItems::RutID(pid) => {
                use crate::schema::collects::dsl::*;
                item_id_vec = collects
                    .filter(&rut_id.eq(&pid))  // limit to 42 inserts, no need paging
                    .select(item_id).load::<String>(conn)?;
            },
            QueryItems::TagID(pid) => {
                use crate::schema::tagitems::dsl::*;
                item_id_vec = tagitems
                    .filter(&tname.eq(&pid))
                    .order(count.desc()).limit(PER_PAGE.into()) // just limit most 
                    .select(item_id).load::<String>(conn)?;
            },
            QueryItems::UserID(pid, f, p) => {
                use crate::schema::staritems::dsl::*;
                let query = staritems.filter(uname.eq(pid)).filter(flag.eq(f));
                item_num = query.clone().count().get_result(conn)?;
                item_id_vec = if p < 1 { 
                    query.order(star_at.desc()).limit(10)
                    .select(item_id).load::<String>(conn)?
                } else {
                    query.order(star_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .select(item_id).load::<String>(conn)?
                };
            },
            QueryItems::KeyID(k,f,i,p) => { // per keyword from taged, star
                let fr = f.trim();
                match fr {
                    "user" => {  
                        use crate::schema::staritems::dsl::{staritems, uname, flag, item_id};
                        let ids = staritems.filter(&uname.eq(&i))
                            .filter(&flag.eq("done"))  // just search done
                            .select(item_id).load::<String>(conn)?;
                        item_list = items.filter(&title.ilike(&k))
                            .filter(&id.eq(any(&ids)))
                            .order(rut_count.desc()).limit(PER_PAGE.into())
                            .load::<Item>(conn)?;
                    },
                    "tag" => {  // hope never use, to optimaze
                        use crate::schema::tagitems::dsl::{tagitems, tname, item_id};
                        let ids = tagitems.filter(&tname.eq(&i))
                            .select(item_id).load::<String>(conn)?;
                        item_list = items.filter(&title.ilike(&k))
                            .filter(&id.eq(any(&ids)))
                            .order(rut_count.desc()).limit(PER_PAGE.into())
                            .load::<Item>(conn)?;
                    },
                    _ => { // just query per keyword, hope never use 
                        item_list = items.filter(&title.ilike(&k))
                            .order(rut_count.desc()).limit(PER_PAGE.into())
                            .load::<Item>(conn)?;
                    },
                }
            },
        };
        
        if item_id_vec.len() > 0 {
            let mut items_query = 
                items.filter(&id.eq(any(&item_id_vec))).load::<Item>(conn)?;
            item_list.append(&mut items_query);
        }

        let item_count = if item_num <= 0 { 
            item_list.len() 
        } else { 
            item_num as usize
        };
    
        Ok( ItemListMsg { 
            status: 200, 
            message: "Success".to_string(),
            items: item_list.clone(),
            count: item_count,
        })
    }
}

// handle msg from api::item.collect_item
impl Handler<CollectItem> for Dba {
    type Result = Result<CollectMsg, ServiceError>;

    fn handle(&mut self, collect: CollectItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::collects::dsl::*;
        use crate::schema::ruts::dsl::{ruts, id as rid, item_count, logo, renew_at};
        use crate::schema::items::dsl::{items, id as itemid, rut_count, cover};
        let conn = &self.0.get().unwrap();
        
        // get item cover then as rut logo, and check if item exist
        let item_q = items
            .filter(&itemid.eq(&collect.item_id))
            .get_result::<Item>(conn)?;
        
        // to gen item order, curr_item_count + 1, or pass from frontend
        let rutID = collect.clone().rut_id;
        let rut_q = ruts             //query once for select/update
            .filter(&rid.eq(&rutID)) 
            .get_result::<Rut>(conn)?;
        let item_num = (&rut_q).item_count;
        // limit the item_count to 42
        if item_num >= 42 {
            return Err(ServiceError::BadRequest(
                "418: The answer to life the universe and everything".into()
            ))
        }
        
        // new collect
        let uuid_v4 = uuid::Uuid::new_v4();
        let uid = format!("{}", uuid_v4);
        let new_collect = Collect::new(uid, collect);
        let collect_new = diesel::insert_into(collects)
            .values(&new_collect).get_result::<Collect>(conn)?;
        
        // to update the item_count + 1 and logo and renew_at in rut
        diesel::update(&rut_q)
            .set((
                item_count.eq(item_count + 1),
                logo.eq(&item_q.cover),
                renew_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        // to update the rut_count + 1 in item
        diesel::update(&item_q)
            .set(rut_count.eq(rut_count + 1)).execute(conn)?;
    
        Ok( CollectMsg { 
            status: 201, 
            message: "Collected".to_string(),
            collect: collect_new,
        })
    }
}

// handle msg from api::item.update_collect
impl Handler<UpdateCollect> for Dba {
    type Result = Result<CollectMsg, ServiceError>;

    fn handle(&mut self, up_collect: UpdateCollect, _: &mut Self::Context) -> Self::Result {
        use crate::schema::collects::dsl::*;
        let conn = &self.0.get().unwrap();

        let collect_query = collects.filter(&id.eq(&up_collect.id))
            .get_result::<Collect>(conn)?;
        if collect_query.uname != up_collect.uname {
            return Err(ServiceError::Unauthorized)
        }

        let collect_update = 
            diesel::update(&collect_query).set(content.eq(up_collect.content))
                .get_result::<Collect>(conn)?;

        Ok( CollectMsg { 
            status: 201,
            message: "Updated".to_string(),
            collect: collect_update.clone(),
        })
    }
}

// handle msg from api::item.del_collect
impl Handler<DelCollect> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, dc: DelCollect, _: &mut Self::Context) -> Self::Result {
        use crate::schema::collects::dsl::*;
        let conn = &self.0.get().unwrap();
        
        let q_collect = collects.filter(&id.eq(&dc.collect_id))
            .get_result::<Collect>(conn)?;
        
        let query_c = q_collect.clone();
        
        // check permission
        if dc.uname != query_c.uname {
            return Err(ServiceError::Unauthorized)
        }
        // some var to use in re-order
        let order_del = query_c.item_order;
        let rutID = query_c.rut_id;
        let itemID = query_c.item_id;
        
        // perform deletion
        diesel::delete(&q_collect).execute(conn)?;

        // to update the item_count - 1 and renew_at in rut
        use crate::schema::ruts::dsl::{ruts, id as rid, item_count, renew_at};
        let rut_q = 
            ruts.filter(&rid.eq(&rutID)).get_result::<Rut>(conn)?;
        
        let item_num = rut_q.item_count;  // to use in re-order

        diesel::update(&rut_q)
            .set((
                item_count.eq(item_count - 1),
                renew_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        // to update the rut_count - 1 in item
        use crate::schema::items::dsl::{items, id as itemid, rut_count};
        diesel::update(
            items.filter(&itemid.eq(&itemID))
        )
        .set(rut_count.eq(rut_count - 1)).execute(conn)?;
        // to update the item order of collect IF not del last one
        if item_num > order_del {
            let lower = order_del + 1;
            let upper = item_num;
            diesel::update(
                collects.filter(rut_id.eq(rutID))
                        .filter(item_order.between(lower, upper)) // betw, inclusive
            )
            .set(item_order.eq(item_order - 1)).execute(conn)?;
        }

        Ok( Msg{ status: 204, message: "Deleted".to_string(),})
    }
}

// handle msg from api::item.get_collect_list
impl Handler<QueryCollects> for Dba {
    type Result = Result<CollectsMsg, ServiceError>;

    fn handle(&mut self, cid: QueryCollects, _: &mut Self::Context) -> Self::Result {
        use crate::schema::collects::dsl::*;
        let conn = &self.0.get().unwrap();
        
        let mut collect_list: Vec<Collect> = Vec::new();
        match cid {
            QueryCollects::RutID(r) => {
                collect_list = collects.filter(&rut_id.eq(&r)).load::<Collect>(conn)?;
            },
            QueryCollects::ItemID(i,p) => {
                collect_list = 
                if p < 1 { // no limit
                    collects.filter(&item_id.eq(&i)).load::<Collect>(conn)?
                } else {
                    collects.filter(&item_id.eq(&i))
                    .order(collect_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Collect>(conn)?
                };
            },
            QueryCollects::UserID(u,p) => {
                collect_list = 
                if p < 1 { // no limit
                    collects.filter(&uname.eq(&u))
                    .load::<Collect>(conn)?
                } else {
                    collects.filter(&uname.eq(&u))
                    .order(collect_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Collect>(conn)?
                };
            },
        }
        
        Ok( CollectsMsg { 
            status: 200,
            message: "Get".to_string(),
            collects: collect_list,
        })
    }
}

// handle msg from api::item.get_collect
impl Handler<QueryCollect> for Dba {
    type Result = Result<CollectMsg, ServiceError>;

    fn handle(&mut self, cid: QueryCollect, _: &mut Self::Context) -> Self::Result {
        use crate::schema::collects::dsl::*;
        let conn = &self.0.get().unwrap();
        
        let collect_query = 
            collects.filter(&id.eq(&cid.collect_id)).get_result::<Collect>(conn)?;
        
        Ok( CollectMsg{
            status: 200, 
            message: "Get".to_string(),
            collect: collect_query,
        })
    }
}

// handle msg from api::item.star_item
impl Handler<NewStarItem> for Dba {
    type Result = Result<StarItemMsg, ServiceError>;

    fn handle(&mut self, act: NewStarItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::staritems::dsl::*;
        let conn = &self.0.get().unwrap();
        
        // check if star no -> todo -> done
        let check_star = staritems
            .filter(&uname.eq(&act.uname))
            .filter(&item_id.eq(&act.item_id))
            .load::<StarItem>(conn)?.pop();

        if let Some(s) = check_star {
            // if stared, todo -> doing or done
            let si = diesel::update(&s)
                .set(flag.eq(act.flag))
                .get_result::<StarItem>(conn)?;
            // update item done_count + 1 if done
            let flg = si.flag.trim();
            if flg == "Done" || flg == "done" {
                use crate::schema::items::dsl::{items, id as itemid, done_count};
                diesel::update(items.filter(&itemid.eq(&act.item_id)))
                    .set(done_count.eq(done_count + 1))
                    .execute(conn)?;
            }

            Ok( StarItemMsg {
                status: 200, 
                message: flg.to_string(),
                note: si.note, 
                when: si.star_at.to_string(),
            })
        } else {
            // otherwise new star
            let uid = format!("{}", uuid::Uuid::new_v4());
            let new_star = StarItem {
                id: uid,
                uname: act.uname,
                item_id: act.item_id,
                star_at: Utc::now().naive_utc(),
                note: act.note,
                flag: act.flag,
                rate: act.rate,
            };
            let si = diesel::insert_into(staritems).values(&new_star)
                .get_result::<StarItem>(conn)?;
            
            Ok( StarItemMsg { 
                status: 200, 
                message: si.flag, 
                note: si.note, 
                when: si.star_at.to_string() 
            })
        }
    }
}

// handle msg from api::item.star_item_status
impl Handler<StarItemStatus> for Dba {
    type Result = Result<StarItemMsg, ServiceError>;

    fn handle(&mut self, status: StarItemStatus, _: &mut Self::Context) -> Self::Result {
        use crate::schema::staritems::dsl::*;
        let conn = &self.0.get().unwrap();

        let check_status = staritems
            .filter(&uname.eq(&status.uname))
            .filter(&item_id.eq(&status.item_id))
            .load::<StarItem>(conn)?.pop();
        

        match check_status {
            Some(s) => { 
                Ok( StarItemMsg { 
                    status: 200, 
                    message: s.flag, 
                    note: s.note, 
                    when: s.star_at.to_string() 
                }) 
            },
            None => { 
                Ok( StarItemMsg { 
                    status: 200, 
                    message: "Options".to_string(),  // as not star
                    note: "".to_string(), 
                    when: "".to_string()
                }) 
            },
        }
    }
}
