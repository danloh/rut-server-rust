// handle user message

use db::dba::Dba;
use actix_web::{actix::Handler, error, Error};
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl, prelude::PgConnection };
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Utc;
use uuid;

use model::user::{ User, NewUser, SignUser, LogUser };
use model::msg::{ Msgs, LoginMsgs };

// handle msg from api::auth.signup
impl Handler<SignUser> for Dba {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, msg: SignUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;

        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let check_user = users.filter(&uname.eq(&msg.uname))
            .load::<User>(conn)
            .map_err(error::ErrorInternalServerError)?.pop();
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
                    let avatar_url = "http://www.gravatar.com/avatar/1".to_string();
                    let new_user = NewUser {
                        id: &uuid,
                        uname: &msg.uname,
                        password: &hash_password,
                        join_at: Utc::now().naive_utc(),
                        avatar: &avatar_url,
                    };
                    diesel::insert_into(users)
                        .values(&new_user).execute(conn)
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

// handle msg from api::auth.signin
impl Handler<LogUser> for Dba {
    type Result = Result<LoginMsgs, Error>;

    fn handle(&mut self, log_user: LogUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;

        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let mut login_user = users.filter(&uname.eq(&log_user.uname)).load::<User>(conn)
                                  .map_err(error::ErrorInternalServerError)?.pop();
        let lg_user = User::new();
        match login_user {
            Some(mut login_user) => {
                match verify(&log_user.password, &login_user.password) {
                    Ok(valid) => {
                        Ok(LoginMsgs {
                            status: 200,
                            message: "Success".to_string(),
                            token: "".to_string(),
                            login_user: login_user,
                        })
                    },
                    Err(_) => {
                        Ok(LoginMsgs { 
                            status: 500,
                            message: "Somehing Wrong".to_string(),
                            token: "".to_string(),
                            login_user: lg_user,
                        })
                    },
                }
            },
            None => {
                Ok(LoginMsgs { 
                    status: 400,
                    message: "wrong password".to_string(),
                    token: "".to_string(),
                    login_user: lg_user,
                })
            }
        }
    }
}
