// typed model and handle user message

use actix::{ Message };
use actix_web::{ dev::Payload, FromRequest, HttpRequest, Error, error };
use jsonwebtoken::{ decode, encode, Header, Validation };
use chrono::{ Local, NaiveDateTime, Utc, Duration };
use std::convert::From;

use crate::model::{ 
    Validate, test_len_limit, re_test_name, re_test_url, re_test_psw, MID_LEN 
};
use crate::errors::ServiceError;
use crate::model::msg::{ Msg, AuthMsg };
use crate::schema::{ users, follows, timelines };


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

// auth via token
impl FromRequest for CheckUser {
    type Config = ();
    type Error = ServiceError;
    type Future = Result<CheckUser, ServiceError>;

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

impl Validate for RegUser {
    fn validate(&self) -> Result<(), Error> {
        let uname = &self.uname;
        let psw = &self.password;
        let check = re_test_name(uname) && re_test_psw(psw);

        if check { 
            Ok(()) 
        } else { 
            Err(error::ErrorBadRequest("Invalid username or password"))
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

impl Validate for AuthUser {
    fn validate(&self) -> Result<(), Error> {
        let uname = &self.uname;
        let psw = &self.password;
        let check = test_len_limit(uname, 3, 42) && test_len_limit(psw, 8, 18);

        if check { 
            Ok(()) 
        } else { 
            Err(error::ErrorBadRequest("Invalid username or password")) 
        }
    }
}

// as msg in get user by uname
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct QueryUser {
    pub uname: String,
}

impl Message for QueryUser {
    type Result = Result<CheckUser, ServiceError>;
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

impl Validate for UpdateUser {
    fn validate(&self) -> Result<(), Error> {
        let nickname = &self.nickname.trim();
        let nickname_test = 
            if nickname.len() == 0 { true } else { re_test_name(nickname) };
        let avatar = &self.avatar.trim();
        let avatar_test = 
            if avatar.len() == 0 { true } else { re_test_url(avatar) };
        let check_len = 
            test_len_limit(&self.location, 0, MID_LEN);
        let check = nickname_test && avatar_test && check_len;

        if check { 
            Ok(()) 
        } else { 
            Err(error::ErrorBadRequest("Invalid Input"))
        }
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

impl Validate for ChangePsw {
    fn validate(&self) -> Result<(), Error> {
        let check = re_test_psw(&self.new_psw);

        if check { 
            Ok(()) 
        } else { 
            Err(error::ErrorBadRequest("Invalid Password"))
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
