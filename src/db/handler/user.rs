// handle user message

use db::dba::Dba;
use db::model::user::{ User, NewUser, CreateUser };
use actix_web::{actix::Handler, error, Error};
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl, prelude::PgConnection };
use uuid;

impl Handler<CreateUser> for Dba {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_user = NewUser {
            id: &uuid,
            name: &msg.name,
        };

        let conn: &PgConnection = &self.0.get().unwrap();

        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;

        let mut items = users
            .filter(id.eq(&uuid))
            .load::<User>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        Ok(items.pop().unwrap())
    }
}