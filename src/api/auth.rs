// api.auth view handler

use actix::Addr;
use futures::Future;
use actix_web::{
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
    web::{ self, Path, Json, Data }
};

use crate::Dba;
use crate::db::user::{ 
    RegUser, UserID, AuthUser, CheckUser, UpdateUser, ChangePsw,
    encode_token 
};
use crate::db::msg::{ AuthMsg, UserMsg };

use crate::{ MIN_LEN, MAX_UNAME_LEN, MIN_PSW_LEN, ANS_LIMIT };


pub fn signup(
    reg: Json<RegUser>,
    db: Data<Addr<Dba>>
) -> impl Future<Item = HttpResponse, Error = Error> {
    // todo some check

    db.send(reg.into_inner())
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn signin(
    auth: Json<AuthUser>,
    db: Data<Addr<Dba>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // todo some check

    db.send(auth.into_inner())
      .from_err()
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
        }
        Err(err) => Ok(err.error_response()),
    })
}

pub fn get(
    path_uname: Path<String>,
    db: Data<Addr<Dba>>
) -> impl Future<Item = HttpResponse, Error = Error> {
    let uname = path_uname.into_inner();
    db.send(UserID{uname})
      .from_err()
      .and_then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn update(
    db: Data<Addr<Dba>>,
    user: Json<UpdateUser>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    // todo some check

    db.send(user.into_inner())
      .from_err()
      .and_then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn change_psw(
    db: Data<Addr<Dba>>,
    psw: Json<ChangePsw>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    // todo some check

    // println!("{:?}",user.uname);
    db.send(psw.into_inner())
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn auth_token(user: CheckUser) -> HttpResponse {
    HttpResponse::Ok().json(user)
}
