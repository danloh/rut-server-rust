// handle user message

use actix::{ Handler, Message };
use actix_web::{ dev::Payload, FromRequest, HttpRequest, Error };
use diesel::prelude::*;
use jsonwebtoken::{ decode, encode, Header, Validation };
use bcrypt::{ hash, DEFAULT_COST, verify };
use chrono::{ Local, NaiveDateTime, Utc, Duration };
use std::convert::From;
use uuid::Uuid;

use crate::Dba;
use crate::errors::ServiceError;
use crate::db::msg::{ Msg, AuthMsg };
use crate::schema::{ users, follows, timelines };

// ###### user model ################

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="users"]
pub struct User {
    pub id: String,
    pub uname: String,
    pub password: String,
    pub join_at: NaiveDateTime,
    pub avatar: String,
    pub email: String,
    pub intro: String,
    pub location: String,
    pub nickname: String,
}

// User's constructor
impl User {
    pub fn new(id: String, uname: String, password: String) -> Self {
        User {
            id,
            uname,
            password,
            join_at: Utc::now().naive_utc(),
            avatar: "".to_owned(),
            email: "".to_owned(),
            intro: "".to_owned(),
            location: "".to_owned(),
            nickname: "".to_owned(),
        }
    }
}

// return as user info w/o password
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="users"]
pub struct CheckUser {
    pub id: String,
    pub uname: String,
    pub join_at: NaiveDateTime,
    pub avatar: String,
    pub email: String,
    pub intro: String,
    pub location: String,
    pub nickname: String,
}

impl From<User> for CheckUser {
    fn from(user: User) -> Self {
        CheckUser {
            id: user.id,
            uname: user.uname,
            join_at: user.join_at,
            avatar: user.avatar,
            email: user.email,
            intro: user.intro,
            location: user.location,
            nickname: user.nickname,
        }
    }
}

impl Message for CheckUser {
    type Result = Result<Msg, ServiceError>;
}

fn get_secret() -> String {
    dotenv::var("SECRET_KEY").unwrap_or_else(|_| "AHaRdGuESsSeCREkY".into())
}

pub fn encode_token(data: &CheckUser) -> Result<String, ServiceError> {
    let claims = Claims::new(data.id.as_str(), data.uname.as_str());
    encode(
        &Header::default(), 
        &claims, 
        get_secret().as_ref()
    )
    .map_err(|_err| ServiceError::InternalServerError)
}

pub fn decode_token(token: &str) -> Result<CheckUser, ServiceError> {
    decode::<Claims>(
        token, 
        get_secret().as_ref(), 
        &Validation::default()
    )
    .map(|data| Ok(data.claims.into()))
    .map_err(|_err| ServiceError::Unauthorized)?
}

// auth via token
impl FromRequest for CheckUser {
    type Config = ();
    type Error = Error;
    type Future = Result<CheckUser, Error>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_token) = req.headers().get("authorization") {
            if let Ok(auth) = auth_token.to_str() {
               let user: CheckUser = decode_token(auth)?;
               return Ok(user);
            }
        }
        Err(ServiceError::Unauthorized.into())
    }
}

// jwt Token auth: Claim, token
#[derive(Debug,Serialize,Deserialize)]
pub struct Claims {
    pub iss: String,   // issuer
    pub sub: String,   // subject
    pub iat: i64,      // issued at
    pub exp: i64,      // expiry
    pub uid: String,   // user id
    pub uname: String,
}

// claims's constructor
impl Claims {
    pub fn new(uid: &str, uname: &str) -> Self {
        Claims {
            iss: "ruthub".into(),
            sub: "auth".into(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24*5)).timestamp(),
            uid: uid.to_owned(),
            uname: uname.to_owned(),
        }
    }
}

impl From<Claims> for CheckUser {
    fn from(claims: Claims) -> Self {
        CheckUser {
            id: claims.uid,
            uname: claims.uname,
            join_at: Utc::now().naive_utc(), // ??
            avatar: "".to_owned(),
            email: "".to_owned(),
            intro: "".to_owned(),
            location: "".to_owned(),
            nickname: "".to_owned(),
        }
    }
}

// message to sign up user
#[derive(Deserialize,Serialize,Debug)]
pub struct RegUser {
    pub uname: String,
    pub password: String,
    pub confirm: String,
}

impl Message for RegUser {
    type Result = Result<Msg, ServiceError>;
}

pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    // get the hashing cost from the env variable or use default
    let hashing_cost: u32 = match dotenv::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    //println!("{}", &hashing_cost);
    hash(plain, hashing_cost).map_err(|_| ServiceError::InternalServerError)
}

// register/signup user
// handle msg from api::auth.signup
impl Handler<RegUser> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: RegUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get().unwrap();

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

// message to login user
#[derive(Deserialize,Serialize,Debug)]
pub struct AuthUser {
    pub uname: String,
    pub password: String,
}

impl Message for AuthUser {
    type Result = Result<CheckUser, ServiceError>;
}

// login / signin
// handle msg from api::auth.signin, auth psw
impl Handler<AuthUser> for Dba {
    type Result = Result<CheckUser, ServiceError>;

    fn handle(&mut self, msg: AuthUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();

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

// as msg in get user by uname
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserID {
    pub uname: String,
}

impl Message for UserID {
    type Result = Result<CheckUser, ServiceError>;
}

// get user
// handle msg from api::auth.get_user
impl Handler<UserID> for Dba {
    type Result = Result<CheckUser, ServiceError>;

    fn handle(&mut self, uid: UserID, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();

        let query_user = 
            users.filter(&uname.eq(&uid.uname)).get_result::<User>(conn)?;

        Ok(query_user.into())
    }
}

// message to update user
#[derive(Deserialize,Serialize,Debug,Clone,AsChangeset)]
#[table_name="users"]
pub struct UpdateUser {
    pub uname: String,  // cannot change, just as id
    pub avatar: String,
    pub email: String,
    pub intro: String,
    pub location: String,
    pub nickname: String,
}

impl Message for UpdateUser {
    type Result = Result<CheckUser, ServiceError>;
}

// edit user
// handle msg from api::auth.update_user
impl Handler<UpdateUser> for Dba {
    type Result = Result<CheckUser, ServiceError>;

    fn handle(&mut self, user: UpdateUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();

        let update_user = diesel::update(users.filter(&uname.eq(&user.uname)))
            .set(&user).get_result::<User>(conn)?;

        Ok(update_user.into())
    }
}

// msg to change psw
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChangePsw {
    pub old_psw: String,
    pub new_psw: String,
    pub uname: String,
}

impl Message for ChangePsw {
    type Result = Result<Msg, ServiceError>;
}

// handle msg from api::auth.change_psw
impl Handler<ChangePsw> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, psw: ChangePsw, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();

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

// to do:
// User follow 
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="follows"]
pub struct Follow {
    pub id: String,
    pub uname: String, // who follow
    pub fname: String,  // who be followed, cannot be uname
    pub fo_at: NaiveDateTime,
    pub note: String,
}

// user's activity record
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="timelines"]
pub struct Timeline {
    pub id: String,
    pub uname: String, // who
    pub action: String, // how: create, star, follow..
    pub obj: String,   // what: rut, item, user, tag..
    pub objid: String,   // what id
    pub act_at: NaiveDateTime, // when
}
