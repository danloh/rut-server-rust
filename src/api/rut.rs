// api.rut, view handler

use futures::Future;
use actix_web::{
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
    web::{ self, Path, Json, Data, Query }
};

use crate::DbAddr;
use crate::INPUT_LIMIT;
use crate::api::{ ReqQuery };
use crate::model::user::{ CheckUser };
use crate::model::rut::{ 
    CreateRut, QueryRut, QueryRuts, UpdateRut, StarOrRut, StarRutStatus 
};

// "/ruts" POST
pub fn new(
    db: Data<DbAddr>,
    rut: Json<CreateRut>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    // check authed via user:FromRequest
    //println!("req: {:?} and user: {:?}", req, user);
    // todo some check, length of input

    db.send(rut.into_inner())
      .from_err()
      .and_then(|res| match res {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(err) => Ok(err.error_response()),
    })
}

// "/ruts/{slug}" GET
pub fn get(
    r_slug: Path<String>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let rut_slug = r_slug.into_inner();
    db.send(QueryRut{rut_slug})
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

// "/ruts/{per}/{perid}?page=p&flag=create|star&kw= fr=" GET
pub fn get_list(
    db: Data<DbAddr>,
    pq: Query<ReqQuery>,
    per_info: Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // extract Path
    let per = per_info.0.trim();
    let perid = per_info.clone().1;
    // extract Query
    let page = pq.page;
    let flag = pq.clone().flag;
    let kw = pq.clone().kw;
    let fr = pq.clone().fr;
    
    let query_msg = match per {
        "item" => QueryRuts::ItemID(perid, page),
        "tag" => QueryRuts::TagID(perid, page),
        "user" => QueryRuts::UserID(perid, flag, page),
        "key" => QueryRuts::KeyID(kw, fr, perid, page), // &keyword=&from=tag|user|item
        _ => QueryRuts::Index(String::from("index")),
    };

    db.send(query_msg)
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn update(
    db: Data<DbAddr>,
    rut: Json<UpdateRut>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    // todo some check

    db.send(rut.into_inner())
      .from_err()
      .and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn star_or_unstar(
    db: Data<DbAddr>,
    star_info: Path<(String, u8, String)>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    let rut_id = star_info.clone().0;
    let action: u8 = star_info.1;
    let note = star_info.clone().2;
    let uname = auth.uname;
    
    db.send( StarOrRut{ rut_id, uname, note, action })
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn star_status(
    db: Data<DbAddr>,
    r_info: Path<String>,
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    let uname = auth.uname;
    let rut_id = r_info.into_inner();
    
    db.send( StarRutStatus { uname, rut_id })
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

// todo
// pub fn delete() {}
