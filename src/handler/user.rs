// handle user message

use db::dba::Dba;
use actix_web::{ actix::Handler, error, Error };
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl, prelude::PgConnection };
use bcrypt::{DEFAULT_COST, hash, verify};
use jwt::{ encode, Header };
use chrono::Utc;
use uuid;
use dotenv;

use model::user::{ 
    User, UserID, NewUser, SignUser, LogUser, CheckUser, UpdateUser, ChangePsw, Claims 
};
use model::msg::{ Msg, LoginMsg };

// handle msg from api::auth.signup
impl Handler<SignUser> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, msg: SignUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;

        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let check_user = users.filter(&uname.eq(&msg.uname))
            .load::<User>(conn)
            .map_err(error::ErrorInternalServerError)?.pop();
        match check_user {
            Some(user) => {
                Ok( Msg {
                    status: 409,
                    message: "Duplicated".to_string(),
                })
            },
            None => {
                use db::schema::users::dsl::*;
                // hash password
                let hash_password = hash(&msg.password, DEFAULT_COST)
                        .map_err(error::ErrorInternalServerError)?;
                // generae uuid as user.id
                let uid = format!("{}", uuid::Uuid::new_v4());
                // prepare insertable data struct as insert_into.value
                let new_user = NewUser {
                    id: &uid,
                    uname: &msg.uname,
                    password: &hash_password,
                    join_at: Utc::now().naive_utc(),
                    avatar: "",
                    email: "",
                    intro: "",
                    location: "",
                    nickname: &msg.uname,
                };
                diesel::insert_into(users)
                    .values(&new_user).execute(conn)
                    .map_err(error::ErrorInternalServerError)?;

                Ok(Msg { 
                    status: 200,
                    message : "Success".to_string(),
                })
            },
        }
    }
}

impl Handler<CheckUser> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, msg: CheckUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let check_user = users.filter(&uname.eq(&msg.uname))
            .load::<User>(conn)
            .map_err(error::ErrorInternalServerError)?.pop();

        if let Some(_) = check_user {
            return Ok(Msg { 
                        status: 409,
                        message : "Occupied".to_string(),
                    });
        }
        Ok(Msg { 
            status: 200,
            message : "Bingo".to_string(),
        })
    }
}

// handle msg from api::auth.signin, auth psw
impl Handler<LogUser> for Dba {
    type Result = Result<LoginMsg, Error>;

    fn handle(&mut self, login: LogUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;

        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let mut query_user = users.filter(&uname.eq(&login.uname)).load::<User>(conn)
                        .map_err(error::ErrorInternalServerError)?.pop();
        let lg_user = User::new("","");

        if let Some(check_user) = query_user {
            match verify(&login.password, &check_user.password) {
                Ok(valid) if valid => {
                    // generate token
                    let claims = Claims::new(&check_user.id, &check_user.uname);
                    let secret_key: String = dotenv::var("SECRET_KEY")
                                            .expect("AHaRdGuESsSeCREkY");
                    let token = encode(&Header::default(), &claims, secret_key.as_ref())
                                .map_err(error::ErrorInternalServerError)?;

                    Ok(LoginMsg {
                        status: 200,
                        message: "Success".to_string(),
                        token: token,
                        exp: 5,  // unit: day
                        user: check_user.into(),
                    })
                },
                _ => {
                    Ok(LoginMsg { 
                        status: 401,
                        message: "Somehing Wrong".to_string(),
                        token: "".to_string(),
                        exp: 0,
                        user: lg_user,
                    })
                },
            }
        } else {
            Ok(LoginMsg { 
                status: 400,
                message: "wrong password".to_string(),
                token: "".to_string(),
                exp: 0,
                user: lg_user,
            })
        }
    }
}

// handle msg from api::auth.get_user
impl Handler<UserID> for Dba {
    type Result = Result<LoginMsg, Error>;

    fn handle(&mut self, uid: UserID, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let query_user = users.filter(&uname.eq(&uid.uname))
            .get_result::<User>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok(LoginMsg {
            status: 200,
            message: "Success".to_string(),
            token: "None".to_string(),  // just for placehold
            exp: 0,
            user: query_user.into(),
        })
    }
}

// handle msg from api::auth.update_user
impl Handler<UpdateUser> for Dba {
    type Result = Result<LoginMsg, Error>;

    fn handle(&mut self, user: UpdateUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let update_user = diesel::update(users.filter(&uname.eq(&user.uname)))
            .set( UpdateUser{
                uname: user.uname.clone(),  // unique, no change
                avatar: user.avatar.clone(),
                email: user.email.clone(),
                intro: user.intro.clone(),
                location: user.location.clone(),
                nickname: user.nickname.clone(),
            })
            .get_result::<User>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok(LoginMsg {
            status: 200,
            message: "Updated".to_string(),
            token: "".to_string(),
            exp: 0,
            user: update_user.into(),
        })
    }
}

// handle msg from api::auth.change_psw
impl Handler<ChangePsw> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, psw: ChangePsw, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let check_user = users.filter(&uname.eq(&psw.uname)).load::<User>(conn)
            .map_err(error::ErrorInternalServerError)?.pop();
        
        if let Some(old) = check_user {
            match verify(&psw.old_psw, &old.password) {
                Ok(valid) if valid => {
                    // hash psw then update
                    let hash_password = hash(&psw.new_psw, DEFAULT_COST)
                        .map_err(error::ErrorInternalServerError)?;
                    diesel::update(&old)
                        .set(password.eq(hash_password)).execute(conn)
                        .map_err(error::ErrorInternalServerError)?;

                    Ok(Msg {
                        status: 200,
                        message: "Success".to_string(),
                    })
                },
                _ => {
                    Ok(Msg { 
                        status: 401,
                        message: "Somehing Wrong".to_string(),
                    })
                },
            }
        } else {
            Ok(Msg { 
                status: 404,
                message: "No Existing".to_string(),
            })
        }
    }
}
