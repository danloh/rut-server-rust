// api.index

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Path, Json, State 
};
use futures::Future;
use router::AppState;
use db::model::user::CreateUser;

pub fn hello((user, state): (Json<CreateUser>, State<AppState>)) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `Dba` actor
    state.db.send(CreateUser {
            uname: user.uname.clone(),
            password: user.password.clone(),
            confirm_password: user.confirm_password.clone(),
        })
        .from_err().and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
