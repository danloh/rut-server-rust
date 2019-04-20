// api.etc, view handler: comment, excerpt, etc.

use futures::Future;
use actix_web::{
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
    web::{ self, Path, Json, Data, Query }
};

use crate::DbAddr;
use crate::INPUT_LIMIT;
use crate::api::{ ReqQuery, re_test_url, len_limit };
use crate::model::user::{ CheckUser };
use crate::model::etc::{ Etc, PostEtc, QueryEtcs };

pub fn new(
    db: Data<DbAddr>,
    petc: Json<PostEtc>, 
    auth: CheckUser
) -> impl Future<Item = HttpResponse, Error = Error> {

    db.send( PostEtc {
        content: petc.content.clone(),
        post_to: petc.post_to.clone(),
        to_id: petc.to_id.clone(),
        uname: auth.uname,
    })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(err) => Ok(err.error_response()),
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
    let page = pq.page;
    
    db.send( QueryEtcs{ per, perid, page })
      .from_err()
      .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}
