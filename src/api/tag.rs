// api.tag, view handler

use actix_web::{
    web::{self, Data, Json, Path, Query},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};

use crate::api::ReqQuery;
use crate::model::tag::{
    CheckTag, QueryTags, RutTag, StarOrTag, 
    StarTagStatus, Tag, TagAny, TagRut, UpdateTag,
};
use crate::model::user::CheckUser;
use crate::model::{replace_sep_tag, Validate, TAG_LEN};
use crate::DbAddr;

pub fn new(
    db: Data<DbAddr>,
    tg: Path<String>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let tname = replace_sep_tag(tg.into_inner().trim(), "-");
    let action = String::from("POST");

    let tag = CheckTag { tname, action };

    result(tag.validate())
        .from_err()
        .and_then(move |_| db.send(tag).from_err())
        .and_then(|res| match res {
            Ok(t) => Ok(HttpResponse::Ok().json(t)),
            Err(e) => Ok(e.error_response()),
        })
}

// new_tag Post, get_tag Get, 2 api send msg to a same msg handler

pub fn get(db: Data<DbAddr>, tg: Path<String>) -> impl Future<Item = HttpResponse, Error = Error> {
    let tname = tg.into_inner();
    let action = String::from("GET");

    db.send(CheckTag { tname, action })
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

    db.send(tg_msg).from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn update(
    db: Data<DbAddr>,
    tg: Json<UpdateTag>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let tag = tg.into_inner();
    // todo some check
    let p_name = tag.pname.trim();
    let pname = if p_name == "" {
        "".to_owned()
    } else {
        replace_sep_tag(p_name, "-")
    };
    let up_tag = UpdateTag { pname, ..tag };

    result(up_tag.validate())
        .from_err()
        .and_then(move |_| db.send(up_tag).from_err())
        .and_then(|res| match res {
            Ok(t) => Ok(HttpResponse::Ok().json(t)),
            Err(e) => Ok(e.error_response()),
        })
}

// to be deprecated
pub fn tag_rut(
    db: Data<DbAddr>,
    rutg: Json<RutTag>,
    tg_info: Path<(u8, String)>, // ?? no use?
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let tags = rutg.into_inner();

    // filter per length, no whitespace; todo: regex to test tag name
    let tnames: Vec<String> = tags
        .tnames
        .clone()
        .into_iter()
        .map(|t| replace_sep_tag(t.trim(), "-"))
        .filter(|t| t.len() <= TAG_LEN && t.len() >= 1)
        .collect();

    let rut_tags = RutTag { tnames, ..tags };

    result(rut_tags.validate())
        .from_err()
        .and_then(move |_| db.send(rut_tags).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

// for tag rut|item|etc
pub fn tag_any(
    db: Data<DbAddr>,
    tg: Json<TagAny>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let tags = tg.into_inner();

    // filter per length, no whitespace; todo: regex to test tag name
    let tnames: Vec<String> = tags
        .tnames
        .clone()
        .into_iter()
        .map(|t| replace_sep_tag(t.trim(), "-"))
        .filter(|t| t.len() <= TAG_LEN && t.len() >= 1)
        .collect();

    let any_tags = TagAny { tnames, ..tags };

    result(any_tags.validate())
        .from_err()
        .and_then(move |_| db.send(any_tags).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn star_or_unstar(
    db: Data<DbAddr>,
    star_info: Path<(String, u8, String)>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let tname = star_info.clone().0;
    let action: u8 = star_info.1;
    let note = star_info.clone().2;

    db.send(StarOrTag {
        uname: auth.uname,
        tname,
        note,
        action,
    })
    .from_err()
    .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn star_status(
    db: Data<DbAddr>,
    tg: Path<String>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let uname = auth.uname;
    let tname = tg.into_inner();

    db.send(StarTagStatus { uname, tname })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(err) => Ok(err.error_response()),
        })
}
