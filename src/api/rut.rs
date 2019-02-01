// api.rut

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State 
};
use futures::Future;
use router::AppState;
use model::rut::CreateRut;

pub fn new_rut((rut, state): (Json<CreateRut>, State<AppState>))
 -> FutureResponse<HttpResponse> {
    state.db.send(CreateRut {
        title: rut.title.clone(),
        url: rut.url.clone(),
        content: rut.content.clone(),
        user_id: rut.user_id.clone(),
        user_intro: rut.user_intro.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}