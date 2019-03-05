// api.auth view handler

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    FromRequest, Error, error, Json, State
};
use futures::Future;
use jwt::{decode, Validation};
use router::AppState;
use model::user::{ 
    User, UserID, SignUser, LogUser, CheckUser, UpdateUser, ChangePsw, Claims 
};

pub fn signup((sign, req): (Json<SignUser>, HttpRequest<AppState>))
 -> FutureResponse<HttpResponse> {
    // do some check
    let l_u = sign.uname.trim().len();
    let l_p = sign.password.trim().len();
    let l_rp = sign.confirm_password.trim().len();
    let check: bool = l_u > 0 && l_u <= 16 && l_p >= 8 && l_p == l_rp;
    if !check {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send(SignUser{
        uname: sign.uname.clone(),
        password: sign.password.clone(),
        confirm_password: sign.confirm_password.clone(),
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

pub fn signin((login, req): (Json<LogUser>, HttpRequest<AppState>))
 -> FutureResponse<HttpResponse> {
    // do some check
    let l_u = login.uname.trim().len();
    let l_p = login.password.trim().len();
    let check: bool = l_u > 0 && l_u <= 16 && l_p >= 8;
    if !check {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send(LogUser{
        uname: login.uname.clone(),
        password: login.password.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}

// auth token
pub fn decode_token(token: &str) -> Result<CheckUser, Error> {
    let secret_key: String = dotenv::var("SECRET_KEY")
                    .expect("AHaRdGuESsSeCREkY");
    decode::<Claims>(token, secret_key.as_ref(), &Validation::default())
        .map(|data| Ok(data.claims.into()))
        .map_err(error::ErrorUnauthorized)?
}

impl<S> FromRequest<S> for CheckUser {
    type Config = ();
    type Result = Result<CheckUser, Error>;
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        // println!("From Auth_Token: {:?}", req); 
        if let Some(auth_token) = req.headers().get("authorization") {
            if let Ok(i) = auth_token.to_str() {
               let user: CheckUser = decode_token(i)?;
               return Ok(user);
            }
        }
        Err(error::ErrorUnauthorized(401))
    }
}

pub fn auth_token(user: CheckUser) -> HttpResponse {
    HttpResponse::Ok().json(user)
}

pub fn get_user(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let userid = String::from(req.match_info().get("userid").unwrap());
    req.state().db.send( UserID{userid})
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}

pub fn update_user((user, req, auth): (Json<UpdateUser>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // do some check
    let l_u = user.uname.trim().len();
    let check = auth.id != user.id || l_u > 16 || l_u == 0;
    if check {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send( UpdateUser{
        id: auth.id.clone(),
        uname: user.uname.clone(),
        avatar: user.avatar.clone(),
        email: user.email.clone(),
        intro: user.intro.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}

pub fn change_psw((psw, req, user): (Json<ChangePsw>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // do some check
    let userid = String::from(req.match_info().get("userid").unwrap());
    let l_p = psw.new_psw.trim().len();
    let check = user.id != userid || l_p < 8;
    if check {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send( ChangePsw{
        old_psw: psw.old_psw.clone(),
        new_psw: psw.new_psw.clone(),
        user_id: user.id.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}
