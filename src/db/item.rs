// item typed model and msg handler

use actix::Handler;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{
    self, dsl::any, ExpressionMethods, 
    PgTextExpressionMethods, QueryDsl, RunQueryDsl
};
use uuid::Uuid;

use crate::bot::WebPage;
use crate::errors::ServiceError;
use crate::model::item::{
    Collect, CollectItem, DelCollect, Item, NewItem, 
    NewStarItem, QueryCollect, QueryCollects, QueryItem, 
    QueryItems, StarItem, StarItemStatus, UpdateCollect, UpdateItem,
};
use crate::model::msg::{CollectMsg, CollectsMsg, ItemListMsg, ItemMsg, Msg, StarItemMsg};
use crate::model::rut::Rut;
use crate::model::PER_PAGE;
use crate::util::share::gen_slug;
use crate::Dba;

// handle msg from api::item.submit_item
impl Handler<NewItem> for Dba {
    type Result = Result<ItemMsg, ServiceError>;

    fn handle(&mut self, submit: NewItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items::dsl::*;
        let conn = &self.0.get()?;

        // check if existing, field may be ""
        // do not use or_filter()
        let s_uiid = &submit.uiid;
        let s_url = &submit.url;
        if s_uiid.trim() != "" {
            let check_uid = items.filter(&uiid.eq(s_uiid)).load::<Item>(conn)?.pop();
            if let Some(i) = check_uid {
                return Ok(ItemMsg {
                    status: 422,
                    message: "Existing".to_string(),
                    item: i,
                });
            }
        }

        if s_url.trim() != "" {
            let check_url = items.filter(&url.eq(s_url)).load::<Item>(conn)?.pop();
            if let Some(i) = check_url {
                return Ok(ItemMsg {
                    status: 422,
                    message: "Existing".to_string(),
                    item: i,
                });
            }
        }

        let uuid_v4 = uuid::Uuid::new_v4();
        let uid = format!("{}", uuid_v4);
        let i_slug = gen_slug("i", &submit.title, &uuid_v4);
        let new_item = Item::new(uid, i_slug, submit);
        let item_new = diesel::insert_into(items)
            .values(&new_item)
            .get_result::<Item>(conn)?;

        Ok(ItemMsg {
            status: 201,
            message: "Submitted".to_string(),
            item: item_new,
        })
    }
}

// handle msg from api::item.update_item
impl Handler<UpdateItem> for Dba {
    type Result = Result<ItemMsg, ServiceError>;

    fn handle(&mut self, item: UpdateItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items::dsl::*;
        let conn = &self.0.get()?;

        let old_item = items.filter(&id.eq(&item.id)).get_result::<Item>(conn)?;
        // to update slug if title changed
        let i_slug = if item.title != old_item.title {
            let i_uuid = Uuid::parse_str(&old_item.id)?;
            gen_slug("i", &item.title, &i_uuid)
        } else {
            old_item.clone().slug
        };

        let item_update = diesel::update(&old_item)
            .set((
                title.eq(item.title),
                uiid.eq(item.uiid),
                authors.eq(item.authors),
                pub_at.eq(item.pub_at),
                publisher.eq(item.publisher),
                category.eq(item.category),
                url.eq(item.url),
                cover.eq(item.cover),
                edition.eq(item.edition),
                detail.eq(item.detail),
                slug.eq(i_slug),
            ))
            .get_result::<Item>(conn)?;

        Ok(ItemMsg {
            status: 201,
            message: "Updated".to_string(),
            item: item_update,
        })
    }
}

// handle msg from api::item.get_item
impl Handler<QueryItem> for Dba {
    type Result = Result<ItemMsg, ServiceError>;

    fn handle(&mut self, islug: QueryItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items::dsl::*;
        let conn = &self.0.get()?;

        let item_query = items
            .filter(&slug.eq(&islug.item_slug)) // slug here only
            .get_result::<Item>(conn)?;

        Ok(ItemMsg {
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
        let conn = &self.0.get()?;

        let mut item_id_vec: Vec<String> = Vec::new();
        let mut item_list: Vec<Item> = Vec::new();
        let mut item_num = 0; // total

        // better do some limit
        match perid {
            QueryItems::ItemID(i) => {
                item_list = items.filter(&id.eq(&i)).load::<Item>(conn)?;
            }
            QueryItems::Title(t) => {
                item_list = items
                    .filter(&title.ilike(&t)) // ilike: %k%, %k, k%
                    .or_filter(&uiid.ilike(&t))
                    .limit(10)
                    .load::<Item>(conn)?;
            }
            QueryItems::Uiid(d) => {
                item_list = items
                    .filter(&uiid.ilike(&d))
                    .or_filter(&title.ilike(&d))
                    .limit(10)
                    .load::<Item>(conn)?;
            }
            QueryItems::ItemUrl(u) => {
                // query in db or via spider
                // url 1to1 item
                let item = items.filter(&url.ilike(&u)).load::<Item>(conn)?.pop();
                match item {
                    Some(i) => {
                        item_list = vec![i];
                    }
                    None => {
                        item_list = Vec::new();
                        // spider per url  // issue alert !!
                        //println!("via spider");
                        /* let page = WebPage::new(&u);
                        let sp_item = page.into_item();
                        // insert new to db
                        let uuid_v4 = uuid::Uuid::new_v4();
                        let uid = format!("{}", uuid_v4);
                        let i_slug = gen_slug("i", &sp_item.title, &uuid_v4);
                        let new_item = Item::new(uid, i_slug, sp_item);
                        let item_new = diesel::insert_into(items)
                            .values(&new_item)
                            .get_result::<Item>(conn)?;
                        item_list = vec![item_new]; */
                    }
                }
            }
            QueryItems::RutID(pid) => {
                use crate::schema::collects::dsl::*;
                item_id_vec = collects
                    .filter(&rut_id.eq(&pid)) // limit to 42 inserts, no need paging
                    .select(item_id)
                    .load::<String>(conn)?;
            }
            QueryItems::TagID(pid) => {
                use crate::schema::tagitems::dsl::*;
                item_id_vec = tagitems
                    .filter(&tname.eq(&pid))
                    .order(count.desc())
                    .limit(PER_PAGE.into()) // just limit most
                    .select(item_id)
                    .load::<String>(conn)?;
            }
            QueryItems::UserID(pid, f, p) => {
                use crate::schema::staritems::dsl::*;
                let query = staritems.filter(uname.eq(pid)).filter(flag.eq(f));
                item_num = query.clone().count().get_result(conn)?;
                item_id_vec = if p < 1 {
                    query
                        .order(star_at.desc())
                        .limit(10)
                        .select(item_id)
                        .load::<String>(conn)?
                } else {
                    query
                        .order(star_at.desc())
                        .limit(PER_PAGE.into())
                        .offset((PER_PAGE * (p - 1)).into())
                        .select(item_id)
                        .load::<String>(conn)?
                };
            }
            QueryItems::KeyID(k, f, i, p) => {
                // per keyword from taged, star
                let fr = f.trim();
                match fr {
                    "user" => {
                        use crate::schema::staritems::dsl::{flag, item_id, staritems, uname};
                        let ids = staritems
                            .filter(&uname.eq(&i))
                            .filter(&flag.eq(3)) // just search done
                            .select(item_id)
                            .load::<String>(conn)?;
                        item_list = items
                            .filter(&title.ilike(&k))
                            .filter(&id.eq(any(&ids)))
                            .order(rut_count.desc())
                            .limit(PER_PAGE.into())
                            .load::<Item>(conn)?;
                    }
                    "tag" => {
                        // hope never use, to optimaze
                        use crate::schema::tagitems::dsl::{item_id, tagitems, tname};
                        let ids = tagitems
                            .filter(&tname.eq(&i))
                            .select(item_id)
                            .load::<String>(conn)?;
                        item_list = items
                            .filter(&title.ilike(&k))
                            .filter(&id.eq(any(&ids)))
                            .order(rut_count.desc())
                            .limit(PER_PAGE.into())
                            .load::<Item>(conn)?;
                    }
                    _ => {
                        // just query per keyword, hope never use
                        item_list = items
                            .filter(&title.ilike(&k))
                            .order(rut_count.desc())
                            .limit(PER_PAGE.into())
                            .load::<Item>(conn)?;
                    }
                }
            }
        };

        if item_id_vec.len() > 0 {
            let mut items_query = items.filter(&id.eq(any(&item_id_vec))).load::<Item>(conn)?;
            item_list.append(&mut items_query);
        }

        let item_count = if item_num <= 0 {
            item_list.len()
        } else {
            item_num as usize
        };

        Ok(ItemListMsg {
            status: 200,
            message: "Success".to_string(),
            items: item_list,
            count: item_count,
        })
    }
}

// handle msg from api::item.collect_item
impl Handler<CollectItem> for Dba {
    type Result = Result<CollectMsg, ServiceError>;

    fn handle(&mut self, collect: CollectItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::collects::dsl::*;
        use crate::schema::items::dsl::{cover, id as itemid, items, rut_count};
        use crate::schema::ruts::dsl::{id as rid, item_count, logo, renew_at, ruts};
        let conn = &self.0.get()?;

        // to check if have collected
        let check_collect = collects
            .filter(&rut_id.eq(&collect.rut_id))
            .filter(&item_id.eq(&collect.item_id))
            .load::<Collect>(conn)?
            .pop();
        if let Some(c) = check_collect {
            return Err(ServiceError::BadRequest("400: Duplicate".into()));
        }

        // get item cover then as rut logo, and check if item exist
        let item_q = items
            .filter(&itemid.eq(&collect.item_id))
            .get_result::<Item>(conn)?;

        // to gen item order, curr_item_count + 1, or pass from frontend
        let rutID = collect.clone().rut_id;
        let rut_q = ruts //query once for select/update
            .filter(&rid.eq(&rutID))
            .get_result::<Rut>(conn)?;
        let item_num = (&rut_q).item_count;
        // limit the item_count to 42
        if item_num >= 42 {
            return Err(ServiceError::BadRequest("418: Answer 42".into()));
        }

        // new collect
        let uuid_v4 = uuid::Uuid::new_v4();
        let uid = format!("{}", uuid_v4);
        let i_order = (item_num + 1) as i16;
        let new_collect = Collect::new(uid, i_order, collect);
        let collect_new = diesel::insert_into(collects)
            .values(&new_collect)
            .get_result::<Collect>(conn)?;

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
            .set(rut_count.eq(rut_count + 1))
            .execute(conn)?;

        Ok(CollectMsg {
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
        let conn = &self.0.get()?;

        let collect_query = collects
            .filter(&id.eq(&up_collect.id))
            .get_result::<Collect>(conn)?;
        if collect_query.uname != up_collect.uname {
            return Err(ServiceError::Unauthorized);
        }

        let collect_update = diesel::update(&collect_query)
            .set(content.eq(up_collect.content))
            .get_result::<Collect>(conn)?;

        Ok(CollectMsg {
            status: 201,
            message: "Updated".to_string(),
            collect: collect_update,
        })
    }
}

// handle msg from api::item.del_collect
impl Handler<DelCollect> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, dc: DelCollect, _: &mut Self::Context) -> Self::Result {
        use crate::schema::collects::dsl::*;
        let conn = &self.0.get()?;

        let q_collect = collects
            .filter(&id.eq(&dc.collect_id))
            .get_result::<Collect>(conn)?;

        let query_c = q_collect.clone();

        // check permission
        if dc.uname != query_c.uname {
            return Err(ServiceError::Unauthorized);
        }
        // some var to use in re-order
        let order_del = query_c.item_order;
        let rutID = query_c.rut_id;
        let itemID = query_c.item_id;

        // perform deletion
        diesel::delete(&q_collect).execute(conn)?;

        // to update the item_count - 1 and renew_at in rut
        use crate::schema::ruts::dsl::{id as rid, item_count, renew_at, ruts};
        let rut_q = ruts.filter(&rid.eq(&rutID)).get_result::<Rut>(conn)?;

        let item_num = rut_q.item_count as i16; // to use in re-order

        diesel::update(&rut_q)
            .set((
                item_count.eq(item_count - 1),
                renew_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        // to update the rut_count - 1 in item
        use crate::schema::items::dsl::{id as itemid, items, rut_count};
        diesel::update(items.filter(&itemid.eq(&itemID)))
            .set(rut_count.eq(rut_count - 1))
            .execute(conn)?;
        // to update the item order of collect IF not del last one
        if item_num > order_del {
            let lower = order_del + 1;
            let upper = item_num;
            diesel::update(
                collects
                    .filter(rut_id.eq(rutID))
                    .filter(item_order.between(lower, upper)), // betw, inclusive
            )
            .set(item_order.eq(item_order - 1))
            .execute(conn)?;
        }

        Ok(Msg {
            status: 204,
            message: "Deleted".to_string(),
        })
    }
}

// handle msg from api::item.get_collect_list
impl Handler<QueryCollects> for Dba {
    type Result = Result<CollectsMsg, ServiceError>;

    fn handle(&mut self, cid: QueryCollects, _: &mut Self::Context) -> Self::Result {
        use crate::schema::collects::dsl::*;
        let conn = &self.0.get()?;

        let mut collect_list: Vec<Collect> = Vec::new();
        match cid {
            QueryCollects::RutID(r) => {
                collect_list = collects.filter(&rut_id.eq(&r)).load::<Collect>(conn)?;
            }
            QueryCollects::ItemID(i, p) => {
                collect_list = if p < 1 {
                    // no limit
                    collects.filter(&item_id.eq(&i)).load::<Collect>(conn)?
                } else {
                    collects
                        .filter(&item_id.eq(&i))
                        .order(collect_at.desc())
                        .limit(PER_PAGE.into())
                        .offset((PER_PAGE * (p - 1)).into())
                        .load::<Collect>(conn)?
                };
            }
            QueryCollects::UserID(u, p) => {
                collect_list = if p < 1 {
                    // no limit
                    collects.filter(&uname.eq(&u)).load::<Collect>(conn)?
                } else {
                    collects
                        .filter(&uname.eq(&u))
                        .order(collect_at.desc())
                        .limit(PER_PAGE.into())
                        .offset((PER_PAGE * (p - 1)).into())
                        .load::<Collect>(conn)?
                };
            }
        }

        Ok(CollectsMsg {
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
        let conn = &self.0.get()?;

        let collect_query = collects
            .filter(&id.eq(&cid.collect_id))
            .get_result::<Collect>(conn)?;

        Ok(CollectMsg {
            status: 200,
            message: "Get".to_string(),
            collect: collect_query,
        })
    }
}

// handle msg from api::item.star_item
impl Handler<NewStarItem> for Dba {
    type Result = Result<StarItemMsg, ServiceError>;

    fn handle(&mut self, istar: NewStarItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::staritems::dsl::*;
        let conn = &self.0.get()?;

        // check if star-ed already
        let check_star = staritems
            .filter(&uname.eq(&istar.uname))
            .filter(&item_id.eq(&istar.item_id))
            .load::<StarItem>(conn)?
            .pop();

        // flag
        let flg = istar.flag;
        let mut si: StarItem;

        if let Some(s) = check_star {
            // if stared, just update flag:  todo -> doing -> done
            si = diesel::update(&s)
                .set((note.eq(&istar.note), flag.eq(&flg), rate.eq(&istar.rate)))
                .get_result::<StarItem>(conn)?;
            // update item done_count + 1 if done
            if flg == 3 {
                use crate::schema::items::dsl::{done_count, id as itemid, items};
                diesel::update(items.filter(&itemid.eq(&istar.item_id)))
                    .set(done_count.eq(done_count + 1))
                    .execute(conn)?;
            }
        } else {
            // otherwise new star-item
            let uid = format!("{}", uuid::Uuid::new_v4());
            let new_star = StarItem {
                id: uid,
                uname: istar.uname,
                item_id: istar.item_id,
                star_at: Utc::now().naive_utc(),
                note: istar.note,
                flag: flg,
                rate: istar.rate,
            };
            si = diesel::insert_into(staritems)
                .values(&new_star)
                .get_result::<StarItem>(conn)?;
        }

        Ok(StarItemMsg {
            status: 200,
            message: si.flag.to_string(),
            note: si.note,
            when: si.star_at.to_string(),
        })
    }
}

// handle msg from api::item.star_item_status
impl Handler<StarItemStatus> for Dba {
    type Result = Result<StarItemMsg, ServiceError>;

    fn handle(&mut self, status: StarItemStatus, _: &mut Self::Context) -> Self::Result {
        use crate::schema::staritems::dsl::*;
        let conn = &self.0.get()?;

        let check_status = staritems
            .filter(&uname.eq(&status.uname))
            .filter(&item_id.eq(&status.item_id))
            .load::<StarItem>(conn)?
            .pop();

        match check_status {
            Some(s) => Ok(StarItemMsg {
                status: 200,
                message: s.flag.to_string(),
                note: s.note,
                when: s.star_at.to_string(),
            }),
            None => {
                Ok(StarItemMsg {
                    status: 200,
                    message: "Options".to_string(), // as not star
                    note: "".to_string(),
                    when: "".to_string(),
                })
            }
        }
    }
}
