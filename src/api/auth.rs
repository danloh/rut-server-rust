// api.auth view handler

use futures::{ future::result, Future };
use actix_web::{
   HttpRequest, HttpResponse, Responder, 
    error, Error, ResponseError,
    web::{ self, Path, Json, Data }
};

use crate::DbAddr;
use crate::model::user::{ 
    RegUser, QueryUser, AuthUser, CheckUser, UpdateUser, ChangePsw, encode_token
};
use crate::model::Validate;
use crate::model::msg::{ AuthMsg, UserMsg };


pub fn signup(
    reg_user: Json<RegUser>,
    db: Data<DbAddr>
) -> impl Future<Item = HttpResponse, Error = Error> {
    
    let reg = reg_user.into_inner();

    result(reg.validate()).from_err()
        .and_then(
            move |_| db.send(reg).from_err()
        )
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn signin(
    auth: Json<AuthUser>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    
    let auth_user = auth.into_inner();

    result(auth_user.validate()).from_err()
        .and_then(
            move |_| db.send(auth_user).from_err()
        )
        .and_then(|res| match res {
            Ok(user) => {
                let token = encode_token(&user)?;
                let auth_msg = AuthMsg{
                    status: 200,
                    message: "Success".to_string(),
                    token: token,
                    exp: 5,  // unit: day
                    user: user,
                };
                Ok(HttpResponse::Ok().json(auth_msg))
            },
            Err(e) => Ok(e.error_response()),
    })
}

pub fn get(
    path_uname: Path<String>,
    db: Data<DbAddr>
) -> impl Future<Item = HttpResponse, Error = Error> {
    let uname = path_uname.into_inner();
    db.send(QueryUser{uname})
      .from_err()
      .and_then(|res| match res {
        Ok(user) => {
          let user_msg = UserMsg{
              status: 200,
              message: "Success".to_string(),
              user: user,
          };
          Ok(HttpResponse::Ok().json(user_msg))
        },
        Err(er) => Ok(er.error_response()),
    })
}

pub fn update(
    db: Data<DbAddr>,
    user: Json<UpdateUser>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    
    let up_user = user.into_inner();

    // auth.uname == user.uname
    if auth.uname != up_user.uname {
        panic!("No Permission"); // to have a better way!!
    }

    result(up_user.validate()).from_err()
        .and_then(
            move |_| db.send(up_user).from_err()
        )
        .and_then(|res| match res {
            Ok(u) => {
                let token = encode_token(&u)?;
                let auth_msg = AuthMsg{
                    status: 200,
                    message: "Success".to_string(),
                    token: token,
                    exp: 5,  // unit: day
                    user: u,
                };
                Ok(HttpResponse::Ok().json(auth_msg))
            },
            Err(e) => Ok(e.error_response()),
    })
}

pub fn change_psw(
    db: Data<DbAddr>,
    psw: Json<ChangePsw>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    
    let user_psw = psw.into_inner();

    // auth.uname == user.uname
    if auth.uname != user_psw.uname {
        panic!("No Permission"); // to have a better way!!
    }

    result(user_psw.validate()).from_err().and_then(
        move |_| db.send(user_psw).from_err()
    )
    .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(e) => Ok(e.error_response()),
    })
}

pub fn auth_token(user: CheckUser) -> HttpResponse {
    HttpResponse::Ok().json(user)
}
