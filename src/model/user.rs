// user model

use db::schema::users;
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime, Duration, Local};
use model::msg::{ Msg, LoginMsg };

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
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
}

// User's constructor
impl User {
    pub fn new(id: &str, uname: &str) -> CheckUser {
        CheckUser {
            id: id.to_owned(),
            uname: uname.to_owned(),
            join_at: Utc::now().naive_utc(),
            avatar: "".to_owned(),
            email: "".to_owned(),
            intro: "".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub uname: &'a str,
    pub password: &'a str,
    pub join_at: NaiveDateTime,
    pub avatar: &'a str,
    pub email: &'a str,
    pub intro: &'a str,
}

// message to check username, or return as user info
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="users"]
pub struct CheckUser {
    pub id: String,
    pub uname: String,
    pub join_at: NaiveDateTime,
    pub avatar: String,
    pub email: String,
    pub intro: String,
}

impl Message for CheckUser {
    type Result = Result<Msg, Error>;
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
        }
    }
}

// message to sign up user
#[derive(Deserialize, Serialize, Debug)]
pub struct SignUser {
    pub uname: String,
    pub password: String,
    pub confirm_password: String,
}

impl Message for SignUser {
    type Result = Result<Msg, Error>;
}

// message to login user
#[derive(Deserialize, Serialize, Debug)]
pub struct LogUser {
    pub uname: String,
    pub password: String,
}

impl Message for LogUser {
    type Result = Result<LoginMsg, Error>;
}

// as msg in get user by id
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserID {
    pub userid: String,
}

impl Message for UserID {
    type Result = Result<LoginMsg, Error>;
}

// message to update user
#[derive(Deserialize,Serialize,Debug,Clone,AsChangeset)]
#[table_name="users"]
pub struct UpdateUser {
    pub id: String,
    pub uname: String,
    pub avatar: String,
    pub email: String,
    pub intro: String,
}

impl Message for UpdateUser {
    type Result = Result<LoginMsg, Error>;
}

// msg to change psw
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChangePsw {
    pub old_psw: String,
    pub new_psw: String,
    pub uname: String,
}

impl Message for ChangePsw {
    type Result = Result<Msg, Error>;
}

//////////////////////////
// jwt util: Claim, token

#[derive(Debug, Serialize, Deserialize)]
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
            exp: (Local::now() + Duration::hours(24)).timestamp(),
            uid: uid.to_owned(),
            uname: uname.to_owned(),
        }
    }
}


// per userid to query rut, item, tag, to do
// struct PerUser { userID: (String, String, String)} // e.g. id, rut, create
