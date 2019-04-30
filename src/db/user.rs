// typed model and handle user message

use actix::{ Handler };
use diesel::prelude::*;
use bcrypt::{ hash, DEFAULT_COST, verify };
use uuid::Uuid;

use crate::Dba;
use crate::errors::ServiceError;
use crate::model::msg::{ Msg, AuthMsg };
use crate::model::user::{ 
    User, CheckUser, RegUser, AuthUser, QueryUser, UpdateUser, ChangePsw 
};


pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    // get the hashing cost from the env variable or use default
    let hashing_cost: u32 = match dotenv::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    hash(plain, hashing_cost).map_err(|_| ServiceError::InternalServerError)
}

// register/signup user
// handle msg from api::auth.signup
impl Handler<RegUser> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: RegUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let check_user = users.filter(&uname.eq(&msg.uname))
            .load::<User>(conn)?.pop();
        match check_user {
            Some(_) => {
                Ok( Msg{ status: 409, message: "Duplicated".to_string(),})
            },
            None => {
                // hash password
                let pswd: String = hash_password(&msg.password)?;
                // generae uuid as user.id
                let uid: String = format!("{}", uuid::Uuid::new_v4());
                let unm: String  = msg.uname;
                let new_user = User::new(uid, unm, pswd);
                diesel::insert_into(users).values(&new_user).get_result::<User>(conn)?;

                Ok( Msg{ status: 201, message : "Success".to_string(),} )
            },
        }
    }
}

// login / signin
// handle msg from api::auth.signin, auth psw
impl Handler<AuthUser> for Dba {
    type Result = Result<CheckUser, ServiceError>;

    fn handle(&mut self, msg: AuthUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let mut query_user = 
            users.filter(&uname.eq(&msg.uname)).load::<User>(conn)?.pop();

        if let Some(check_user) = query_user {
            match verify(&msg.password, &check_user.password) {
                Ok(valid) if valid => {
                    return Ok(check_user.into());
                },
                _ => (),
            }
        } 
        Err(ServiceError::BadRequest("Auth Failed".into(),))
    }
}

// get user
// handle msg from api::auth.get_user
impl Handler<QueryUser> for Dba {
    type Result = Result<CheckUser, ServiceError>;

    fn handle(&mut self, uid: QueryUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let query_user = 
            users.filter(&uname.eq(&uid.uname)).get_result::<User>(conn)?;

        Ok(query_user.into())
    }
}

// edit user
// handle msg from api::auth.update_user
impl Handler<UpdateUser> for Dba {
    type Result = Result<CheckUser, ServiceError>;

    fn handle(&mut self, user: UpdateUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let update_user = diesel::update(users.filter(&uname.eq(&user.uname)))
            .set(&user).get_result::<User>(conn)?;

        Ok(update_user.into())
    }
}

// handle msg from api::auth.change_psw
impl Handler<ChangePsw> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, psw: ChangePsw, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let check_user = 
            users.filter(&uname.eq(&psw.uname)).load::<User>(conn)?.pop();
        
        if let Some(old) = check_user {
            match verify(&psw.old_psw, &old.password) {
                Ok(valid) if valid => {
                    // hash psw then update
                    let new_password: String = hash_password(&psw.new_psw)?;
                    diesel::update(&old)
                        .set(password.eq(new_password)).execute(conn)?;

                    Ok( Msg{ status: 200, message: "Success".to_string(),})
                },
                _ => {
                    Ok( Msg{ status: 401, message: "Somehing Wrong".to_string(),})
                },
            }
        } else {
            Ok( Msg{ status: 404, message: "No Existing".to_string(),})
        }
    }
}
