// item msg handler

use db::dba::Dba;
use actix_web::{ actix::Handler, error, Error };
use diesel::{ self, dsl::any, QueryDsl, ExpressionMethods, PgTextExpressionMethods, RunQueryDsl };
use chrono::Utc;
use uuid;

use model::item::{
    Item, NewItem, SubmitItem, UpdateItem, ItemID, ItemsPerID, Collect, NewCollect, 
    CollectItem, CollectID, CollectIDs, UpdateCollect, DelCollect, 
    StarItem, NewStarItem, ItemStar, StarItemStatus  
};
use model::msg::{ Msg, ItemMsg, ItemListMsg, CollectMsg, CollectsMsg };
use model::rut::Rut;
use PER_PAGE;

// handle msg from api::item.submit_item
impl Handler<SubmitItem> for Dba {
    type Result = Result<ItemMsg, Error>;

    fn handle(&mut self, submit: SubmitItem, _: &mut Self::Context) -> Self::Result {
        use db::schema::items::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        // check if existing, field may be ""
        let s_uiid = &submit.uiid;
        let s_url = &submit.url;
        if s_uiid.trim() != "" {
            let check_q_id = items.filter(&uiid.eq(s_uiid))
                .load::<Item>(conn).map_err(error::ErrorInternalServerError)?.pop();
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
                .load::<Item>(conn).map_err(error::ErrorInternalServerError)?.pop();
            if let Some(i) = check_q_id {
                return Ok( ItemMsg { 
                    status: 422, 
                    message: "Existing".to_string(),
                    item: i,
                })
            }
        }

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_item = NewItem {
            id: &uuid,
            title: &submit.title,
            uiid: &submit.uiid,
            authors: &submit.authors,  
            pub_at: &submit.pub_at,   
            publisher: &submit.publisher,
            category: &submit.category, 
            url: &submit.url,
            cover: &submit.cover,
            edition: &submit.edition,
            detail: &submit.detail,
            rut_count: 0,
            etc_count: 0, 
            done_count: 0, 
        };
        let item_new = diesel::insert_into(items)
            .values(&new_item)
            .get_result::<Item>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok( ItemMsg { 
            status: 200, 
            message: "Submitted".to_string(),
            item: item_new.clone(),
        })
    }
}

// handle msg from api::item.get_item
impl Handler<ItemID> for Dba {
    type Result = Result<ItemMsg, Error>;

    fn handle(&mut self, itemid: ItemID, _: &mut Self::Context) -> Self::Result {
        use db::schema::items::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let item_query = items.filter(&id.eq(&itemid.item_id))
            .load::<Item>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let mut item = Item::new(); 
        match item_query {
            Some(q) => {
                item = q.clone();
            },
            None => (),
        }
    
        Ok( ItemMsg { 
            status: 200, 
            message: "Success".to_string(),
            item: item,
        })
    }
}

// handle msg from api::item.get_item_list
impl Handler<ItemsPerID> for Dba {
    type Result = Result<ItemListMsg, Error>;

    fn handle(&mut self, perid: ItemsPerID, _: &mut Self::Context) -> Self::Result {
        use db::schema::items::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let mut item_id_vec: Vec<String> = Vec::new();
        let mut item_list: Vec<Item> = Vec::new();
        
        // better do some limit
        match perid {
            ItemsPerID::ItemID(i) => {
                item_list = items
                    .filter(&id.eq(&i))
                    .load::<Item>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            ItemsPerID::Title(t) => {
                item_list = items
                    .filter(&title.ilike(&t)) // ilike: %k%, %k, k%
                    .or_filter(&uiid.ilike(&t)).limit(10)
                    .load::<Item>(conn) 
                    .map_err(error::ErrorInternalServerError)?;
            },
            ItemsPerID::Uiid(d) => {
                item_list = items
                    .filter(&uiid.ilike(&d))
                    .or_filter(&title.ilike(&d)).limit(10)
                    .load::<Item>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            ItemsPerID::ItemUrl(u) => {
                item_list = items
                    .filter(&url.ilike(&u)).limit(10)
                    .load::<Item>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            ItemsPerID::RutID(pid) => {
                use db::schema::collects::dsl::*;
                item_id_vec = collects
                    .filter(&rut_id.eq(&pid))  // limit to 42 inserts, no need paging
                    .select(item_id).load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            ItemsPerID::TagID(pid) => {
                use db::schema::tagitems::dsl::*;
                item_id_vec = tagitems
                    .filter(&tname.eq(&pid))
                    .order(count.desc()).limit(PER_PAGE.into()) // just limit most 
                    .select(item_id).load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            ItemsPerID::UserID(pid, f, p) => {
                use db::schema::staritems::dsl::*;
                item_id_vec = if p < 1 { 
                    staritems.filter(&uname.eq(&pid))
                    .filter(&flag.eq(&f))  
                    .order(star_at.desc()).limit(10)
                    .select(item_id).load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?
                } else {
                    staritems.filter(&uname.eq(&pid))
                    .filter(&flag.eq(&f))  // f = todo or done
                    .order(star_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .select(item_id).load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?
                };
            },
        };
        
        if item_id_vec.len() > 0 {
            let mut items_query = items.filter(&id.eq(any(&item_id_vec)))
                .load::<Item>(conn).map_err(error::ErrorInternalServerError)?;
            item_list.append(&mut items_query);
        }
    
        Ok( ItemListMsg { 
            status: 200, 
            message: "Success".to_string(),
            items: item_list.clone(),
            count: item_list.clone().len(),
        })
    }
}

// handle msg from api::item.update_item
impl Handler<UpdateItem> for Dba {
    type Result = Result<ItemMsg, Error>;

    fn handle(&mut self, item: UpdateItem, _: &mut Self::Context) -> Self::Result {
        use db::schema::items::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let item_update = diesel::update(items.filter(&id.eq(&item.id)))
            .set( &UpdateItem{
                id: item.id.clone(),
                title: item.title.clone(),
                uiid: item.uiid.clone(),
                authors: item.authors.clone(),
                pub_at: item.pub_at.clone(),
                publisher: item.publisher.clone(),
                category: item.category.clone(),
                url: item.url.clone(),
                cover: item.cover.clone(),
                edition: item.edition.clone(), 
                detail: item.detail.clone(),
            })
            .get_result::<Item>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok( ItemMsg { 
            status: 200,
            message: "Updated".to_string(),
            item: item_update.clone(),
        })
    }
}

// handle msg from api::item.collect_item
impl Handler<CollectItem> for Dba {
    type Result = Result<CollectMsg, Error>;

    fn handle(&mut self, collect: CollectItem, _: &mut Self::Context) -> Self::Result {
        use db::schema::collects::dsl::*;
        use db::schema::ruts::dsl::{ruts, id as rid, item_count, logo, renew_at};
        use db::schema::items::dsl::{items, id as itemid, rut_count, cover};
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        // get item cover then as rut logo, and check if item exist
        let item_q = items
            .filter(&itemid.eq(&collect.item_id))
            .get_result::<Item>(conn)
            .map_err(error::ErrorInternalServerError)?;
        
        // to gen item order, curr_item_count + 1, or pass from frontend
        let rutID = collect.rut_id;
        let rut_q = ruts             //query once for select/update
            .filter(&rid.eq(&rutID)) 
            .get_result::<Rut>(conn)
            .map_err(error::ErrorInternalServerError)?;
        let item_num = (&rut_q).item_count;
        // limit the item_count to 42
        if item_num >= 42 {
            return Ok( CollectMsg { 
                        status: 418, 
                        message: "The answer to life the universe and everything".to_string(),
                        collect: Collect::new(),
                    })
        }

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_collect = NewCollect {
            id: &uuid,
            rut_id: &rutID,
            item_id: &item_q.id, // ok?
            item_order: item_num + 1,
            content: &collect.content,
            uname: &collect.uname,
            collect_at: Utc::now().naive_utc(),
        };
        let collect_new = diesel::insert_into(collects)
            .values(&new_collect).get_result::<Collect>(conn)
            .map_err(error::ErrorInternalServerError)?;
        
        // to update the item_count + 1 and logo and renew_at in rut
        diesel::update(&rut_q)
            .set((
                item_count.eq(item_count + 1),
                logo.eq(&item_q.cover),
                renew_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)
            .map_err(error::ErrorInternalServerError)?;
        // to update the rut_count + 1 in item
        diesel::update(&item_q)
            .set(rut_count.eq(rut_count + 1)).execute(conn)
            .map_err(error::ErrorInternalServerError)?;
    
        Ok( CollectMsg { 
            status: 200, 
            message: "Collected".to_string(),
            collect: collect_new,
        })
    }
}

// handle msg from api::item.get_collect_list
impl Handler<CollectIDs> for Dba {
    type Result = Result<CollectsMsg, Error>;

    fn handle(&mut self, cid: CollectIDs, _: &mut Self::Context) -> Self::Result {
        use db::schema::collects::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let mut collect_list: Vec<Collect> = Vec::new();
        match cid {
            CollectIDs::RutID(r) => {
                collect_list = collects.filter(&rut_id.eq(&r))
                    .load::<Collect>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            CollectIDs::ItemID(i,p) => {
                collect_list = if p < 1 {
                    collects.filter(&item_id.eq(&i))
                    .load::<Collect>(conn)
                    .map_err(error::ErrorInternalServerError)?
                } else {
                    collects.filter(&item_id.eq(&i))
                    .order(collect_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Collect>(conn)
                    .map_err(error::ErrorInternalServerError)?
                };
            },
            CollectIDs::UserID(u,p) => {
                collect_list = if p < 1 {
                    collects.filter(&uname.eq(&u))
                    .load::<Collect>(conn)
                    .map_err(error::ErrorInternalServerError)?
                } else {
                    collects.filter(&uname.eq(&u))
                    .order(collect_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .load::<Collect>(conn)
                    .map_err(error::ErrorInternalServerError)?
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
impl Handler<CollectID> for Dba {
    type Result = Result<CollectMsg, Error>;

    fn handle(&mut self, cid: CollectID, _: &mut Self::Context) -> Self::Result {
        use db::schema::collects::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let c_query = collects
            .filter(&id.eq(&cid.collect_id)).load::<Collect>(conn)
            .map_err(error::ErrorInternalServerError)?.pop();
        
        if let Some(c) = c_query {
            Ok( CollectMsg { 
                status: 200, 
                message: "Get".to_string(),
                collect: c,
            })
        } else {
            Ok( CollectMsg { 
                status: 404, 
                message: "Nothing".to_string(),
                collect: Collect::new(),
            })
        }    
    }
}

// handle msg from api::item.del_collect
impl Handler<DelCollect> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, dc: DelCollect, _: &mut Self::Context) -> Self::Result {
        use db::schema::collects::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let q_c = collects.filter(&id.eq(&dc.collect_id))
            .get_result::<Collect>(conn)
            .map_err(error::ErrorInternalServerError)?;
        
        // to use in re-order
        let q_c_c = q_c.clone();
        let order_del = q_c_c.item_order;
        let rutid = q_c_c.rut_id;
        
        if dc.uname == q_c.uname {
            diesel::delete(&q_c).execute(conn)
                .map_err(error::ErrorInternalServerError)?;

            // to update the item_count - 1 and renew_at in rut
            use db::schema::ruts::dsl::{ruts, id as rid, item_count, renew_at};
            let r_q = ruts.filter(&rid.eq(&dc.rut_id))
                .get_result::<Rut>(conn).map_err(error::ErrorInternalServerError)?;
            
            let item_num = r_q.item_count;  // to use in re-order

            diesel::update(&r_q)
                .set((
                    item_count.eq(item_count - 1),
                    renew_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)
                .map_err(error::ErrorInternalServerError)?;
            // to update the rut_count - 1 in item
            use db::schema::items::dsl::{items, id as itemid, rut_count};
            diesel::update(items.filter(&itemid.eq(&dc.item_id)))
                .set(rut_count.eq(rut_count - 1)).execute(conn)
                .map_err(error::ErrorInternalServerError)?;
            // to update the item order of collect
            if item_num > order_del {
                let lower = order_del + 1;
                let upper = item_num;
                diesel::update(
                    collects.filter(rut_id.eq(rutid))
                        .filter(item_order.between(lower, upper)) // betw, inclusive
                )
                .set(item_order.eq(item_order - 1)).execute(conn)
                .map_err(error::ErrorInternalServerError)?;
            }

            Ok( Msg { 
                status: 200, 
                message: "Deleted".to_string(),
            })
        } else {
            Ok( Msg { 
                status: 401, 
                message: "No Permission".to_string(),
            })
        }
    }
}

// handle msg from api::item.update_collect
impl Handler<UpdateCollect> for Dba {
    type Result = Result<CollectMsg, Error>;

    fn handle(&mut self, c: UpdateCollect, _: &mut Self::Context) -> Self::Result {
        use db::schema::collects::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let c_q = collects.filter(&id.eq(&c.id))
            .get_result::<Collect>(conn)
            .map_err(error::ErrorInternalServerError)?;
        if c_q.uname != c.uname {
            return Ok( CollectMsg { 
                        status: 401,
                        message: "No Permission ".to_string(),
                        collect: Collect::new(),
                    })
        }

        let collect_update = diesel::update(&c_q)
            .set(content.eq(c.content))
            .get_result::<Collect>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok( CollectMsg { 
            status: 200,
            message: "Updated".to_string(),
            collect: collect_update.clone(),
        })
    }
}

// handle msg from api::item.star_item
impl Handler<NewStarItem> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, act: NewStarItem, _: &mut Self::Context) -> Self::Result {
        use db::schema::staritems::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        // check if star no -> todo -> done
        let check_star = staritems
            .filter(&uname.eq(&act.uname))
            .filter(&item_id.eq(&act.item_id))
            .load::<StarItem>(conn)
            .map_err(error::ErrorInternalServerError)?.pop();

        if let Some(s) = check_star {
            // if stared, todo -> done
            diesel::update(&s)
                .set(flag.eq("done".to_string()))
                .execute(conn)
                .map_err(error::ErrorInternalServerError)?;
            // update item done_count + 1
            use db::schema::items::dsl::{items, id as itemid, done_count};
            diesel::update(items.filter(&itemid.eq(&act.item_id)))
                .set(done_count.eq(done_count + 1)).execute(conn)
                .map_err(error::ErrorInternalServerError)?;

            Ok( Msg { status: 200, message: "done".to_string() })
        } else {
            // otherwise not star, so no -> todo
            let uuid = format!("{}", uuid::Uuid::new_v4());
            let new_star = ItemStar {
                id: &uuid,
                uname: &act.uname,
                item_id: &act.item_id,
                star_at: Utc::now().naive_utc(),
                note: &act.note,
                flag: "todo",
            };
            diesel::insert_into(staritems).values(&new_star)
                .execute(conn).map_err(error::ErrorInternalServerError)?;
            
            Ok( Msg { status: 200, message: "todo".to_string() })
        }
    }
}

// handle msg from api::item.star_item_status
impl Handler<StarItemStatus> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, status: StarItemStatus, _: &mut Self::Context) -> Self::Result {
        use db::schema::staritems::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let check_status = staritems
            .filter(&uname.eq(&status.uname))
            .filter(&item_id.eq(&status.item_id))
            .load::<StarItem>(conn)
            .map_err(error::ErrorInternalServerError)?.pop();
        
        match check_status {
            Some(s) => { Ok( Msg {status: 200, message: s.flag }) },
            None => { Ok( Msg { status: 200, message: "no".to_string() }) },
        }
    }
}
