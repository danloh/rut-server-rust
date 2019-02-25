// item msg handler

use db::dba::Dba;
use actix_web::{ actix::Handler, error, Error };
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl };
use chrono::Utc;
use uuid;

use model::item::{
    Item, NewItem, SubmitItem, UpdateItem, ItemID, ItemIDs,
    Collect, NewCollect, CollectItem 
};
use model::msg::{ Msgs, ItemMsgs, ItemListMsgs, CollectMsgs };
use model::rut::Rut;

// handle msg from api::item.submit_item
impl Handler<SubmitItem> for Dba {
    type Result = Result<ItemMsgs, Error>;

    fn handle(&mut self, submit_item: SubmitItem, _: &mut Self::Context) -> Self::Result {
        use db::schema::items::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_item = NewItem {
            id: &uuid,
            title: &submit_item.title,
            uiid: &submit_item.uiid,   
            pub_at: &submit_item.pub_at,   
            authors: &submit_item.authors,
            publisher: &submit_item.publisher,
            category: &submit_item.category, 
            url: &submit_item.url,
            cover: &submit_item.cover,
            edition: &submit_item.edition,
            detail: &submit_item.detail,
            rut_count: 0,
            etc_count: 0, 
            done_count: 0, 
        };
        let item_new = diesel::insert_into(items)
            .values(&new_item)
            .get_result::<Item>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok( ItemMsgs { 
            status: 200, 
            message: "Submitted".to_string(),
            item: item_new.clone(),
        })
    }
}

// handle msg from api::item.get_item
impl Handler<ItemID> for Dba {
    type Result = Result<ItemMsgs, Error>;

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
            None => { println!("No Result"); },
        }
    
        Ok( ItemMsgs { 
            status: 200, 
            message: "Success".to_string(),
            item: item,
        })
    }
}

// handle msg from api::item.get_item_list
impl Handler<ItemIDs> for Dba {
    type Result = Result<ItemListMsgs, Error>;

    fn handle(&mut self, itemid: ItemIDs, _: &mut Self::Context) -> Self::Result {
        use db::schema::items::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let item_vec = match itemid {
            ItemIDs::ID(i) => items.filter(&id.eq(&i)).load::<Item>(conn),
            ItemIDs::Title(t) => items.filter(&title.eq(&t)).load::<Item>(conn), // to do contains
            ItemIDs::Uiid(d) => items.filter(&uiid.eq(&d)).load::<Item>(conn),
            ItemIDs::Url(u) => items.filter(&url.eq(&u)).load::<Item>(conn),
        };

        let item_query = item_vec.map_err(error::ErrorInternalServerError)?;
    
        Ok( ItemListMsgs { 
            status: 200, 
            message: "Success".to_string(),
            items: item_query.clone(),
            count: item_query.clone().len(),
        })
    }
}

// handle msg from api::item.update_item
impl Handler<UpdateItem> for Dba {
    type Result = Result<ItemMsgs, Error>;

    fn handle(&mut self, item: UpdateItem, _: &mut Self::Context) -> Self::Result {
        use db::schema::items::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let item_update = diesel::update(items)
            .filter(&id.eq(&item.id))
            .set( &UpdateItem{
                id: item.id.clone(),
                title: item.title.clone(),
                uiid: item.uiid.clone(),
                pub_at: item.pub_at.clone(),
                authors: item.authors.clone(),
                publisher: item.publisher.clone(),
                category: item.category.clone(),
                url: item.url.clone(),
                cover: item.cover.clone(),
                edition: item.edition.clone(), 
                detail: item.detail.clone(),
            })
            .get_result::<Item>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok( ItemMsgs { 
            status: 200, 
            message: "Updated".to_string(),
            item: item_update.clone(),
        })
    }
}

// handle msg from api::item.collect_item
impl Handler<CollectItem> for Dba {
    type Result = Result<CollectMsgs, Error>;

    fn handle(&mut self, collect: CollectItem, _: &mut Self::Context) -> Self::Result {
        use db::schema::collects::dsl::*;
        use db::schema::ruts::dsl::{ruts, id as rid, item_count}; 
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        // to gen item order, curr_item_count + 1, or pass fron frontend
        let rutID = collect.rut_id;
        let item_num = ruts.filter(&rid.eq(&rutID)).select(item_count)
                        .first::<i32>(conn).map_err(error::ErrorInternalServerError)?;
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_collect = NewCollect {
            id: &uuid,
            rut_id: &rutID,
            item_id: &collect.item_id, // need to check??
            item_order: item_num + 1,
            content: &&collect.content,
            // spoiler: bool,  // to do
            creator_id: &&collect.creator_id,
            collect_at: Utc::now().naive_utc(),
        };
        let collect_new = diesel::insert_into(collects)
            .values(&new_collect)
            .get_result::<Collect>(conn)
            .map_err(error::ErrorInternalServerError)?;
    
        Ok( CollectMsgs { 
            status: 200, 
            message: "Collected".to_string(),
            rut_id: rutID.clone(),
            items: vec!(collect_new),
        })
    }
}
