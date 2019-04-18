// api.tag, view handler

use futures::Future;
use actix_web::{
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
    web::{ self, Path, Json, Data, Query }
};

use crate::DbAddr;
use crate::{ MIN_LEN, ANS_LIMIT };
use crate::api::{ ReqQuery, re_test_url, len_limit };
use crate::db::user::{ CheckUser };
use crate::db::tag::{ 
    Tag, CheckTag, UpdateTag, TagsPerID, TagRut, RutTag, 
    StarOrTag, StarTagStatus 
};


pub fn new(
    db: Data<DbAddr>,
    tg: Path<String>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    let tname = tg.into_inner();
    let action = String::from("POST");
    //println!("{:?}", req.method().as_str());
    //println!("{:?}", tname);
    // check the length of tname and inner whitespace
    let l = tname.trim().len();
    if l <= MIN_LEN || l > ANS_LIMIT || tname.contains(" ") {
        panic!("Invalid")
    }

    db.send( CheckTag{ tname, action })
      .from_err()
      .and_then(|res| match res {
        Ok(tag) => Ok(HttpResponse::Ok().json(tag)),
        Err(err) => Ok(err.error_response()),
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
    pq: Query<ReqQuery>,
    per_info: Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // extract Path
    let per = per_info.0.trim();
    let perid = per_info.clone().1;
    
    let tg_msg = match per {
        "rut" => TagsPerID::RutID(perid),
        "item" => TagsPerID::ItemID(perid),
        "tag" => TagsPerID::TagID(perid),
        "user" => TagsPerID::UserID(perid),
        _ => TagsPerID::Index(perid),
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

    db.send(tag)
      .from_err()
      .and_then(|res| match res {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn tag_rut(
    db: Data<DbAddr>,
    rutg: Json<RutTag>,
    tg_info: Path<(String, String)>,
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {

    let action = tg_info.clone().0;     // 0-untag/1-tag
    let rut_id = tg_info.clone().1;

    let tags = rutg.into_inner();

    // filter per length, no inner space; to do: regex to test tag name
    let tnames: Vec<String> = tags.tnames.clone().into_iter().filter(
        |t| t.trim().len() < ANS_LIMIT && t.trim().len() > MIN_LEN && !(t.contains(" "))
    ).collect();
    // check if any
    if tnames.len() == 0  {
        panic!("Invalid")   // temp
    }

    db.send( RutTag{ tnames, rut_id, action })
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
    let tname = star_info.clone().0;
    let action: u8 = star_info.1;
    let note = star_info.clone().2;
    
    db.send( StarOrTag {
        uname: auth.uname,
        tname,
        note,
        action,
    })
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
