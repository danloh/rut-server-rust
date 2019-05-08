// api.etc, view handler: comment, excerpt, etc.

use actix_web::{
    web::{self, Data, Json, Path, Query},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};

use crate::api::ReqQuery;
use crate::model::etc::{Etc, PostEtc, QueryEtcs};
use crate::model::user::CheckUser;
use crate::model::Validate;
use crate::DbAddr;

pub fn new(
    db: Data<DbAddr>,
    petc: Json<PostEtc>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let post_etc = petc.into_inner();
    let uname = auth.uname;
    let new_etc = PostEtc { uname, ..post_etc };

    result(new_etc.validate())
        .from_err()
        .and_then(move |_| db.send(new_etc).from_err())
        .and_then(|res| match res {
            Ok(et) => Ok(HttpResponse::Ok().json(et)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn get_list(
    db: Data<DbAddr>,
    pq: Query<ReqQuery>,
    per_info: Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // extract Path
    let per = per_info.clone().0;
    let perid = per_info.clone().1;
    // extract Query
    let page = std::cmp::max(pq.page, 1);

    db.send(QueryEtcs { per, perid, page })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(err) => Ok(err.error_response()),
        })
}
