// api.tag, view handler

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State
};
use futures::Future;
use router::AppState;
use model::tag::{ 
    Tag, CheckTag, UpdateTag, TagsPerID, NewTagRut, RutTag, 
    StarOrTag, StarTagStatus 
};
use model::user::{ CheckUser };
use api::{ re_test_url, re_test_uname };
use ::{ MIN_LEN, ANS_LIMIT };


pub fn new_tag((req, user): (HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    let tname = String::from(req.match_info().get("tname").unwrap());
    let action = String::from("POST");
    //println!("{:?}", req.method().as_str());
    //println!("{:?}", tname);
    // check the length of tname and inner whitespace
    let l = tname.trim().len();
    if l <= MIN_LEN || l > ANS_LIMIT || tname.contains(" ") {
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
        _ => TagsPerID::Index(perid),
    };

    req.state().db.send(q_per).from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn update_tag((tag, req, user): (Json<UpdateTag>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // do some check
    let url = tag.logo.trim();
    let url_test = if url.len() == 0 { true } else { re_test_url(url) };
    let pname = tag.pname.trim();
    let pname_test = if pname.len() == 0 { true } else { re_test_uname(pname) };
    let tname = String::from(req.match_info().get("tname").unwrap());

    let check = url_test && pname_test && tname == tag.tname;
    if !check {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send( UpdateTag {
        tname: tname.to_string(),  // just as id, not change
        intro: tag.intro.clone(),
        logo: url.to_string(),
        pname: pname.to_string(),
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
    let tnames: Vec<String> = tags.tnames.clone().into_iter().filter(
        |t| t.trim().len() < ANS_LIMIT && t.trim().len() > MIN_LEN && !(t.contains(" "))
    ).collect();
    // check if any
    if tnames.len() == 0  {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send( RutTag { 
        tnames: tnames.clone(),
        rut_id: tags.rut_id.clone(),
        action: action.clone(), 
    })
    .from_err().and_then(|res| match res {
        Ok(rut) => Ok(HttpResponse::Ok().json(rut)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn star_unstar_tag(req: HttpRequest<AppState>, user: CheckUser)
 -> FutureResponse<HttpResponse> {
    let action: u8 = req.match_info().get("action").unwrap().parse().unwrap();
    let tname = String::from(req.match_info().get("tname").unwrap());
    let note = String::from(req.match_info().get("note").unwrap());
    
    req.state().db.send( StarOrTag {
        uname: user.uname.clone(),
        tname: tname.clone(),
        note: note.clone(),
        action: action,
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn star_tag_status(req: HttpRequest<AppState>, user: CheckUser)
 -> FutureResponse<HttpResponse> {
    let uname = user.uname;
    let tname = String::from(req.match_info().get("tname").unwrap());
    
    req.state().db.send( StarTagStatus { uname, tname })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}
