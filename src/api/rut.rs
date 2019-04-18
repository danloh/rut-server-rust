// api.rut, view handler

use actix::Addr;
use futures::Future;
use actix_web::{
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
    web::{ self, Path, Json, Data, Query }
};

use crate::Dba;
use crate::db::rut::{ 
    CreateRut, RutSlug, RutsPerID, UpdateRut, StarOrRut, StarRutStatus 
};
use crate::db::user::{ CheckUser };
use crate::api::{ ReqQuery, re_test_url, len_limit };
use crate::INPUT_LIMIT;

pub fn new(
    db: Data<Addr<Dba>>,
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

pub fn get(
    r_slug: Path<String>,
    db: Data<Addr<Dba>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let rut_slug = r_slug.into_inner();
    db.send(RutSlug{rut_slug})
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

// query: 
pub fn get_list(
    per_info: Path<(String, String)>,
    pq: Query<ReqQuery>,
    db: Data<Addr<Dba>>
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
        "item" => RutsPerID::ItemID(perid, page),
        "tag" => RutsPerID::TagID(perid, page),
        "user" => RutsPerID::UserID(perid, flag, page),
        "key" => RutsPerID::KeyID(kw, fr, perid, page), // &keyword=&from=tag|user|item
        _ => RutsPerID::Index(String::from("index")),
    };

    db.send(query_msg)
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn update(
    db: Data<Addr<Dba>>,
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
    db: Data<Addr<Dba>>,
    star_info: Path<(String, u8, String)>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    let rut_slug = star_info.clone().0;
    let action= star_info.1;
    let note = star_info.clone().2;
    let uname = auth.uname;
    
    db.send( StarOrRut{ rut_slug, uname, note, action })
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn star_status(
    db: Data<Addr<Dba>>,
    r_info: Path<String>,
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    let uname = auth.uname;
    let rut_slug = r_info.into_inner();
    
    db.send( StarRutStatus { uname, rut_slug })
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

// todo
// pub fn delete() {}
