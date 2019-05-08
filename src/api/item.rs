// api.item, view handler

use actix_web::{
    web::{self, Data, Json, Path, Query},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};

use crate::api::ReqQuery;
use crate::model::item::{
    CollectItem, DelCollect, NewItem, NewStarItem, QueryCollect, QueryCollects, QueryItem,
    QueryItems, StarItem, StarItemStatus, UpdateCollect, UpdateItem,
};
use crate::model::user::CheckUser;
use crate::model::Validate;
use crate::model::{re_test_img_url, replace_sep, trim_url_qry};
use crate::DbAddr;

pub fn new(
    db: Data<DbAddr>,
    new_item: Json<NewItem>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let newItem = new_item.into_inner();
    let uid = newItem.uiid;
    let uiid = replace_sep(&uid, "");
    let new_url = newItem.url;
    let url = trim_url_qry(&new_url, "");

    let item_new = NewItem {
        uiid,
        url,
        ..newItem
    };

    result(item_new.validate())
        .from_err()
        .and_then(move |_| db.send(item_new).from_err())
        .and_then(|res| match res {
            Ok(item) => Ok(HttpResponse::Ok().json(item)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn get(
    db: Data<DbAddr>,
    i_slug: Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let item_slug = i_slug.into_inner();
    db.send(QueryItem { item_slug })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
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
    // extract Query
    let page = std::cmp::max(pq.page, 1);
    let flag = pq.clone().flag;
    let kw = pq.clone().kw;
    let fr = pq.clone().fr;

    use base64::decode; // for decode url

    let itemsPerID = match per {
        // hope can fuzzy query per uiid..url, contains
        // here are some issue, 400 or no result, % trimed
        "uiid" => QueryItems::Uiid(perid),
        "title" => QueryItems::Title(perid),
        "url" => QueryItems::ItemUrl(
            String::from_utf8(
                decode(&kw).unwrap_or(Vec::new()), //url as kw, avoid route error
            )
            .unwrap_or("not_url".into()),
        ),
        // query per relations with  rut, tag, user
        "rut" => QueryItems::RutID(perid),
        "tag" => QueryItems::TagID(perid),
        "user" => QueryItems::UserID(perid, flag.parse::<i16>().unwrap_or(3), page),
        "key" => QueryItems::KeyID(kw, fr, perid, page),
        _ => QueryItems::ItemID(perid),
    };

    result(itemsPerID.validate())
        .from_err()
        .and_then(move |_| db.send(itemsPerID).from_err())
        .and_then(|res| match res {
            Ok(items) => Ok(HttpResponse::Ok().json(items)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn update(
    db: Data<DbAddr>,
    up_item: Json<UpdateItem>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let upItem = up_item.into_inner();

    let uid = upItem.uiid;
    let uiid = replace_sep(&uid, "");
    let up_url = upItem.url;
    let url = trim_url_qry(&up_url, "");

    let item_up = UpdateItem {
        uiid,
        url,
        ..upItem
    };

    result(item_up.validate())
        .from_err()
        .and_then(move |_| db.send(item_up).from_err())
        .and_then(|res| match res {
            Ok(item) => Ok(HttpResponse::Ok().json(item)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn collect_item(
    db: Data<DbAddr>,
    c_item: Json<CollectItem>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // todo some check of input

    db.send(c_item.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(item) => Ok(HttpResponse::Ok().json(item)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn get_collect_list(
    db: Data<DbAddr>,
    pq: Query<ReqQuery>,
    per_info: Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // extract Path
    let per = per_info.0.trim();
    let perid = per_info.clone().1;
    // extract Query
    let page = std::cmp::max(pq.page, 1);

    let collectIDs = match per {
        "item" => QueryCollects::ItemID(perid, page),
        "rut" => QueryCollects::RutID(perid),
        "user" => QueryCollects::UserID(perid, page),
        _ => QueryCollects::RutID(perid),
    };

    db.send(collectIDs).from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn get_collect(
    db: Data<DbAddr>,
    cid: Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let collect_id = cid.into_inner();
    let action = "GET".to_string();
    db.send(QueryCollect { collect_id, action })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn del_collect(
    db: Data<DbAddr>,
    cid: Path<String>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // should do some check in frontend

    let collect_id = cid.into_inner();
    let uname = auth.uname; // pass to handler to check permission

    db.send(DelCollect { collect_id, uname })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn update_collect(
    db: Data<DbAddr>,
    up_collect: Json<UpdateCollect>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // need to check the auth_uname == collect_uname, on frontend??
    // check id eque

    db.send(up_collect.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(cmsg) => Ok(HttpResponse::Ok().json(cmsg)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn star_item(
    db: Data<DbAddr>,
    auth: CheckUser,
    star_info: Path<(String, i16, i16, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let item_id = star_info.clone().0;
    let flag = star_info.1; // 1-todo|2-doing|3-done
    let rate = star_info.2;
    let note = star_info.clone().3;
    let uname = auth.uname;

    db.send(NewStarItem {
        uname,
        item_id,
        note,
        flag,
        rate,
    })
    .from_err()
    .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn star_status(
    db: Data<DbAddr>,
    auth: CheckUser,
    itemid: Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let uname = auth.uname;
    let item_id = itemid.into_inner();

    db.send(StarItemStatus { uname, item_id })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(err) => Ok(err.error_response()),
        })
}
