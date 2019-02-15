// api.auth view handler

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State
};
use futures::Future;
use router::AppState;
use model::user::{ User, SignUser, LogUser, CheckUser };

pub fn signup((sign_user, state): (Json<SignUser>, State<AppState>))
 -> FutureResponse<HttpResponse> {
    println!("{:?}", sign_user); 
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

pub fn check_user(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let uname = String::from(req.match_info().get("uname").unwrap());
    req.state().db.send(User::new("", &uname))
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}

pub fn signin((log_user, state, req): (Json<LogUser>, State<AppState>, HttpRequest<AppState>))
 -> FutureResponse<HttpResponse> {
    println!("{:?}", req); 
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

// ??
pub fn auth_token(user: CheckUser) -> HttpResponse {
    HttpResponse::Ok().json(user)
}
