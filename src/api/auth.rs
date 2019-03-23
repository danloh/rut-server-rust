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
use api::{ re_test_uname };
use ::{ MIN_LEN, MAX_UNAME_LEN, MIN_PSW_LEN, ANS_LIMIT };


pub fn signup((sign, req): (Json<SignUser>, HttpRequest<AppState>))
 -> FutureResponse<HttpResponse> {
    // do some check
    let uname = sign.uname.trim();
    let psw = sign.password.clone();
    let repsw = sign.confirm.clone();
    let l_p = psw.trim().len();
    let check = l_p >= MIN_PSW_LEN && psw == repsw && re_test_uname(uname);
    if !check {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send(SignUser{
        uname: uname.to_string(),
        password: psw,
        confirm: repsw,
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
    let uname = login.uname.clone();
    let l_p = login.password.trim().len();
    let check = l_p >= MIN_PSW_LEN && re_test_uname(&uname);
    if !check {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send(LogUser{
        uname: uname,
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
    let uname = String::from(req.match_info().get("uname").unwrap());
    req.state().db.send( UserID{uname})
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}

pub fn update_user((user, req, auth): (Json<UpdateUser>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // do some check
    let nickname = user.nickname.clone();
    let l_u = nickname.trim().len();
    let check = auth.uname != user.uname || l_u > MAX_UNAME_LEN || l_u <= MIN_LEN;
    if check {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send( UpdateUser{
        uname: auth.uname,
        avatar: user.avatar.clone(),
        email: user.email.clone(),
        intro: user.intro.clone(),
        location: user.location.clone(),
        nickname: user.nickname.clone(),
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
    let uname = String::from(req.match_info().get("uname").unwrap());
    let l_p = psw.new_psw.trim().len();
    let check = user.uname != uname || l_p < MIN_PSW_LEN;
    if check {
        use api::gen_response;
        return gen_response(req)
    }
    // println!("{:?}",user.uname);
    req.state().db.send( ChangePsw{
        old_psw: psw.old_psw.clone(),
        new_psw: psw.new_psw.clone(),
        uname: user.uname.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}
