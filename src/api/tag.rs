// api.tag, view handler

use futures::{ future::result, Future};
use actix_web::{
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
    web::{ self, Path, Json, Data, Query }
};

use crate::DbAddr;
use crate::api::{ ReqQuery };
use crate::model::{ Validate, TAG_LEN };
use crate::model::user::{ CheckUser };
use crate::model::tag::{ 
    Tag, CheckTag, UpdateTag, QueryTags, TagRut, RutTag, 
    StarOrTag, StarTagStatus 
};


pub fn new(
    db: Data<DbAddr>,
    tg: Path<String>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    let tname = tg.into_inner().trim().replace(" ", "-");
    let action = String::from("POST");

    let tag = CheckTag{ tname, action };

    result(tag.validate()).from_err()
        .and_then(
            move |_| db.send(tag).from_err()
        )
        .and_then(|res| match res {
            Ok(t) => Ok(HttpResponse::Ok().json(t)),
            Err(e) => Ok(e.error_response()),
        })
}

// new_tag Post, get_tag Get, 2 api send msg to a same msg handler

pub fn get(
    db: Data<DbAddr>,
    tg: Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let tname = tg.into_inner();
    let action = String::from("GET");

    db.send( CheckTag{ tname, action })
      .from_err()
      .and_then(|res| match res {
        Ok(tag) => Ok(HttpResponse::Ok().json(tag)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn get_list(
    db: Data<DbAddr>,
    per_info: Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // extract Path
    let per = per_info.0.trim();
    let perid = per_info.clone().1;
    
    let tg_msg = match per {
        "rut" => QueryTags::RutID(perid),
        "item" => QueryTags::ItemID(perid),
        "tag" => QueryTags::TagID(perid),
        "user" => QueryTags::UserID(perid),
        _ => QueryTags::Index(perid),
    };

    db.send(tg_msg).from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn update(
    db: Data<DbAddr>,
    tg: Json<UpdateTag>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    let tag = tg.into_inner();
    // todo some check
    let p_name = tag.pname.trim();
    let pname = if p_name == "" { "".to_owned() } else { p_name.replace(" ", "-") };
    let up_tag = UpdateTag{ pname, ..tag };

    result(up_tag.validate()).from_err()
        .and_then(
            move |_| db.send(up_tag).from_err()
        )
        .and_then(|res| match res {
            Ok(t) => Ok(HttpResponse::Ok().json(t)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn tag_rut(
    db: Data<DbAddr>,
    rutg: Json<RutTag>,
    tg_info: Path<(u8, String)>, // ?? no use?
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {

    let tags = rutg.into_inner();

    // filter per length, no whitespace; todo: regex to test tag name
    let tnames: Vec<String> = tags.tnames.clone().into_iter()
        .map( |t| t.trim().replace(" ", "-") )
        .filter( |t| t.len() <= TAG_LEN && t.len() >= 1 )
        .collect();

    let rut_tags = RutTag{ tnames, ..tags };

    result(rut_tags.validate()).from_err()
        .and_then(
            move |_| db.send(rut_tags).from_err()
        )
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn star_or_unstar(
    db: Data<DbAddr>,
    star_info: Path<(String, u8, String)>,
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    let tname = star_info.clone().0;
    let action: u8 = star_info.1;
    let note = star_info.clone().2;
    
    db.send( StarOrTag{uname: auth.uname, tname, note, action,} )
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn star_status(
    db: Data<DbAddr>,
    tg: Path<String>,
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    let uname = auth.uname;
    let tname = tg.into_inner();
    
    db.send( StarTagStatus { uname, tname })
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}
