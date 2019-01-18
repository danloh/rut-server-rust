// api.index

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Path, State 
};
use futures::Future;
use router::AppState;
use db::model::user::CreateUser;

pub fn hello((name, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `Dba` actor
    state.db.send(CreateUser {
            name: name.into_inner(),
        })
        .from_err().and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
