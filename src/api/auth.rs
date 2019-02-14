// api.auth view handler

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State
};
use futures::Future;
use router::AppState;
use model::user::{ SignUser, LogUser };

pub fn signup((sign_user, state): (Json<SignUser>, State<AppState>))
 -> FutureResponse<HttpResponse> {
    state.db.send(SignUser{
        uname: sign_user.uname.clone(),
        password: sign_user.password.clone(),
        confirm_password: sign_user.confirm_password.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}

pub fn signin((log_user, state): (Json<LogUser>, State<AppState>))
 -> FutureResponse<HttpResponse> {
    state.db.send(LogUser{
        uname: log_user.uname.clone(),
        password: log_user.password.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}
