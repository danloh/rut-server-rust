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
    //println!("{:?}", req.method().as_str());
    //println!("{:?}", tname);
    // check the length of tname and inner whitespace
    let l = tname.trim().len();
    if l ==0 || l > 16 || tname.contains(" ") {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send( CheckTag { tname, action })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

// new_tag Post, get_tag Get, 2 api send msg to a same msg handler

pub fn get_tag(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
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

pub fn update_tag((tag, req, user): (Json<UpdateTag>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    req.state().db.send( UpdateTag {
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

pub fn tag_rut((tags, req, user): (Json<RutTag>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    let action = String::from(req.match_info().get("action").unwrap()); // 0-untag/1-tag
    let rut_id = String::from(req.match_info().get("rutid").unwrap());
    // println!("{:?}", tags);

    // filter per length, no inner space; to do: regex to test tag name
    let tnames: Vec<String> = tags.tname.clone().into_iter().filter(
        |t| t.trim().len() < 16 && t.trim().len() > 0 && !(t.contains(" "))
    ).collect();
    // check if any
    if tnames.len() == 0  {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send( RutTag { 
        tname: tnames.clone(),
        rut_id: tags.rut_id.clone(),
        action: action.clone(), 
    })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}
