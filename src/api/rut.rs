// api.rut, view handler

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State
};
use futures::Future;
use router::AppState;
use model::rut::{ CreateRut, RutID, RutListType, UpdateRut, StarOrRut };
use model::user::{ CheckUser };

pub fn new_rut((rut, state, user): (Json<CreateRut>, State<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // check authed via user:FromRequest
    state.db.send( CreateRut {
        title: rut.title.clone(),
        url: rut.url.clone(),
        content: rut.content.clone(),
        user_id: user.id.clone(),     // extracted from request as user
        author_id: rut.author_id.clone(),
        credential: rut.credential.clone(), 
    })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn get_rut(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let rut_id = String::from(req.match_info().get("rid").unwrap());
    req.state().db.send(
        RutID{rut_id}
    )
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn get_rut_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let list_type = req.match_info().get("type").unwrap().parse().unwrap();
    let tid = String::from(req.match_info().get("tid").unwrap());
    
    let q_type = match list_type {
        // 0 - user, 1 - item, other - index
        0 => RutListType::UserID(String::from(tid)),
        1 => RutListType::ItemID(String::from(tid)),
        _ => RutListType::Index(String::from("index")),
    };

    req.state().db.send(q_type).from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn update_rut((rut, state, user): (Json<UpdateRut>, State<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
     state.db.send( UpdateRut {
        id: rut.id.clone(),
        title: rut.title.clone(),
        url: rut.url.clone(),
        content: rut.content.clone(),
        author_id: rut.author_id.clone(),
        credential: rut.credential.clone(), 
    })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn star_unstar_rut(req: HttpRequest<AppState>, user: CheckUser)
 -> FutureResponse<HttpResponse> {
    let star_action: u8 = req.match_info().get("action").unwrap().parse().unwrap();
    let rid = String::from(req.match_info().get("rid").unwrap());
    
    req.state().db.send( StarOrRut {
        rut_id: rid.clone(),
        user_id: user.id.clone(),
        action: star_action,
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}
