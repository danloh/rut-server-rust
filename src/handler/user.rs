// handle user message

use db::dba::Dba;
use model::user::{ User, NewUser, CreateUser };
use model::msg::Msgs;
use actix_web::{actix::Handler, error, Error};
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl, prelude::PgConnection };
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Utc;
use uuid;

impl Handler<CreateUser> for Dba {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;
        
        let conn: &PgConnection = &self.0.get().unwrap();
        let check_user = users.filter(&uname.eq(&msg.uname))
                              .load::<User>(conn)
                              .map_err(error::ErrorInternalServerError)?
                              .pop();
        match check_user {
            Some(user) => {
                Ok( Msgs {
                    status: 400,
                    message: "Duplicated".to_string(),
                })
            },
            None => {
                if &msg.password == &msg.confirm_password {
                    use db::schema::users::dsl::*;
                    let hash_password = match hash(&msg.password, DEFAULT_COST) {
                        Ok(h) => h,
                        Err(_) => panic!()
                    };
                    let uuid = format!("{}", uuid::Uuid::new_v4());
                    let avatar_url = "http://www.gravatar.com/avatar/".to_string();
                    let new_user = NewUser {
                        id: &uuid,
                        uname: &msg.uname,
                        password: &hash_password,
                        join_at: Utc::now().naive_utc(),
                        avatar: &avatar_url,
                    };
                    diesel::insert_into(users)
                        .values(&new_user)
                        .execute(conn)
                        .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;
                    Ok(Msgs { 
                        status: 200,
                        message : "Success".to_string(),
                    })
                } else {
                    Ok(Msgs { 
                        status: 400,
                        message : "wrong password".to_string(),
                    })
                }
            }
        }
    }
}
