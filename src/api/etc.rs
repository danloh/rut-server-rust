// api.etc, view handler: comment, excerpt, etc.

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State
};
use futures::Future;
use router::AppState;
use model::etc::{ Etc, PostEtc, EtcsPerID };
use model::user::{ CheckUser };

pub fn post_etc((pe, req, user): (Json<PostEtc>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {

    let l_pe = pe.content.trim().len();
    if l_pe > 320 || l_pe == 0 {
        use api::gen_response;
        return gen_response(req)
    }
    
    req.state().db.send( PostEtc {
        content: pe.content.clone(),
        post_to: pe.post_to.clone(),
        to_id: pe.to_id.clone(),
        uname: user.uname.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn get_etc_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let per = String::from(req.match_info().get("per").unwrap());
    let per_id = String::from(req.match_info().get("perid").unwrap());
    
    req.state().db.send(EtcsPerID{ per, per_id })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}
