// rut msg handler

use db::dba::Dba;
use model::rut::{ Rut, NewRut, CreateRut, RutID, RutListType };
use model::msg::{ Msgs, RutMsgs, RutListMsgs };
use actix_web::{actix::Handler, error, Error};
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl, prelude::PgConnection };
use chrono::Utc;
use uuid;

// handle msg from api::new_rut
impl Handler<CreateRut> for Dba {
    type Result = Result<RutMsgs, Error>;

    fn handle(&mut self, new_rut: CreateRut, _: &mut Self::Context) -> Self::Result {
        use db::schema::ruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_rut = NewRut {
            id: &uuid,
            title: &new_rut.title,
            url: &new_rut.url,
            content: &new_rut.content,
            user_id: &new_rut.user_id,
            user_intro: &new_rut.user_intro,
            create_at: Utc::now().naive_utc(),
            item_count: 0,
            comment_count: 0,
            star_count: 0,
        };
        diesel::insert_into(ruts).values(&new_rut).execute(conn)
                                .map_err(error::ErrorInternalServerError)?;
        let rut_query = ruts.filter(&id.eq(&uuid)).load::<Rut>(conn)
                        .map_err(error::ErrorInternalServerError)?.pop();
        let mut new_r = Rut::new(); 
        match rut_query {
            Some(rut) => {
                new_r = rut.clone();
            },
            None => { println!("No Result"); },
        }
    
        Ok( RutMsgs { 
            status: 200, 
            message: "Success".to_string(),
            rut: new_r,
        })
    }
}

// handle msg from api::get_rut
impl Handler<RutID> for Dba {
    type Result = Result<RutMsgs, Error>;

    fn handle(&mut self, rid: RutID, _: &mut Self::Context) -> Self::Result {
        use db::schema::ruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let rut_query = ruts.filter(&id.eq(&rid.rut_id)).load::<Rut>(conn)
                        .map_err(error::ErrorInternalServerError)?.pop();
        let mut rut = Rut::new(); 
        match rut_query {
            Some(r_q) => {
                rut = r_q.clone();
            },
            None => { println!("No Result"); },
        }
    
        Ok( RutMsgs { 
            status: 200, 
            message: "Success".to_string(),
            rut: rut,
        })
    }
}

// handle msg from api::get_rut_list
impl Handler<RutListType> for Dba {
    type Result = Result<RutListMsgs, Error>;

    fn handle(&mut self, list_type: RutListType, _: &mut Self::Context) -> Self::Result {
        use db::schema::ruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let mut id_list: Vec<String> = Vec::new();
        let mut rut_list: Vec<Rut> = Vec::new();
        
        // build id_list per query type
        match list_type {
            RutListType::Index(_) => { 
                id_list.push(String::from("2bs"));
                id_list.push(String::from("1af"));                     // todo
                id_list.push(String::from("2bsgh"));
            },
            RutListType::UserID(u) => { println!("userid is {}", u); }, // todo
            RutListType::ItemID(i) => { println!("itemid is {}", i); }, // todo
        }
        // build rut_list
        for rid in id_list {
            let rut_query = ruts.filter(&id.eq(&rid)).load::<Rut>(conn)
                            .map_err(error::ErrorInternalServerError)?.pop();
            let mut rut = Rut::new(); 
            match rut_query {
                Some(r_q) => {
                    rut = r_q.clone();
                    rut_list.push(rut);
                },
                None => { println!("Nothing"); },
            }
        }

        Ok( RutListMsgs { 
            status: 200, 
            message: "Success".to_string(),
            ruts: rut_list.clone(),
            count: rut_list.len(),
        })
    }
}
