// api.rut

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State 
};
use futures::Future;
use router::AppState;
use model::rut::CreateRut;

pub fn new_rut((new_rut, state): (Json<CreateRut>, State<AppState>))
 -> FutureResponse<HttpResponse> {
    state.db.send(CreateRut {
        title: new_rut.title,
        url: new_rut.url,
        content: new_rut.content,
        user_id: new_rut.user_id,
        user_intro: new_rut.user_intro,
    })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}