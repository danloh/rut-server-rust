// rut msg handler

use db::dba::Dba;
use model::rut::{ Rut, NewRut, CreateRut };
use model::msg::Msgs;
use actix_web::{actix::Handler, error, Error};
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl, prelude::PgConnection };
use chrono::Utc;
use uuid;

impl Handler<CreateRut> for Dba {
    type Result = Result<Msgs, Error>;

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
    
        Ok( Msgs { status: 200, message : "Success".to_string(),})
    }
}
