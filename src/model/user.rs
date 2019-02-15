// user model

use db::schema::users;
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime, Duration, Local};
use model::msg::{ Msgs, LoginMsgs };

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="users"]
pub struct User {
    pub id: String,
    pub uname: String,
    pub password: String,
    pub join_at: NaiveDateTime,
    pub avatar: String,
}

// User's constructor
impl User {
    pub fn new(id: &str, uname: &str) -> CheckUser {
        CheckUser {
            id: id.to_owned(),
            uname: uname.to_owned(),
            join_at: Utc::now().naive_utc(),
            avatar: "".to_owned(),
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
}

// message to check username, or return as user info
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="users"]
pub struct CheckUser {
    pub id: String,
    pub uname: String,
    pub join_at: NaiveDateTime,
    pub avatar: String,
}

impl Message for CheckUser {
    type Result = Result<Msgs, Error>;
}

impl From<User> for CheckUser {
    fn from(user: User) -> Self {
        CheckUser {
            id: user.id,
            uname: user.uname,
            join_at: user.join_at,
            avatar: user.avatar,
        }
    }
}

impl From<Claims> for CheckUser {
    fn from(claims: Claims) -> Self {
        CheckUser {
            id: claims.uid,
            uname: "".to_owned(),
            join_at: Utc::now().naive_utc(),
            avatar: "".to_owned(),
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
    type Result = Result<Msgs, Error>;
}

// message to login user
#[derive(Deserialize, Serialize, Debug)]
pub struct LogUser {
    pub uname: String,
    pub password: String,
}

impl Message for LogUser {
    type Result = Result<LoginMsgs, Error>;
}


// jwt util: Claim, token

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,   // issuer
    pub sub: String,   // subject
    pub iat: i64,      // issued at
    pub exp: i64,      // expiry
    pub uid: String, // user id
}

// claims's constructor
impl Claims {
    pub fn new(uid: &str) -> Self {
        Claims {
            iss: "ruthub".into(),
            sub: "auth".into(),
            uid: uid.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
}
