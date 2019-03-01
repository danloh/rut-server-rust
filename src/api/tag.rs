// api.tag, view handler

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State
};
use futures::Future;
use router::AppState;
use model::tag::{ Tag, CheckTag, UpdateTag, TagsPerID, NewTagRut, RutTag };
use model::user::{ CheckUser };

pub fn new_tag((req, user): (HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    let tname = String::from(req.match_info().get("tname").unwrap());
    let action = String::from("POST");
    // println!("{:?}", req.method().as_str());
    
    req.state().db.send( CheckTag { tname, action })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn get_tag(req: HttpRequest<AppState>)
 -> FutureResponse<HttpResponse> {
    let tname = String::from(req.match_info().get("tname").unwrap());
    let action = String::from("GET");
    // println!("{:?}", req.method().as_str());

    req.state().db.send( CheckTag { tname, action })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn get_tag_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let per = req.match_info().get("per").unwrap();
    let perid = String::from(req.match_info().get("id").unwrap());
    
    let q_per = match per {
        "rut" => TagsPerID::RutID(perid),
        "item" => TagsPerID::ItemID(perid),
        "tag" => TagsPerID::TagID(perid),
        "user" => TagsPerID::UserID(perid),
        _ => TagsPerID::RutID(perid),
    };

    req.state().db.send(q_per).from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn update_tag((tag, state, user): (Json<UpdateTag>, State<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    state.db.send( UpdateTag {
        tname: tag.tname.clone(),
        intro: tag.intro.clone(),
        logo: tag.logo.clone(),
        pname: tag.pname.clone(), 
    })
    .from_err().and_then(|res| match res {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn tag_rut((tag, req, user): (Json<RutTag>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    let action = String::from(req.match_info().get("action").unwrap()); // 0/1
    let rut_id = String::from(req.match_info().get("rutid").unwrap());
    // println!("{:?}", tag);
    req.state().db.send( RutTag { 
        tname: tag.tname.clone(),
        rut_id: tag.rut_id.clone(),
        action: action.clone(), 
    })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}
