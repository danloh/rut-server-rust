// item msg handler

use db::dba::Dba;
use actix_web::{ actix::Handler, error, Error };
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl };
use chrono::Utc;
use uuid;

use model::item::{ Item, NewItem, SubmitItem, ItemID };
use model::msg::{ Msgs, ItemMsgs };

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
            .load::<Item>(conn)
            .map_err(error::ErrorInternalServerError)?.pop();
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
