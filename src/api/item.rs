// api.item, view handler

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State
};
use futures::Future;
use router::AppState;
use model::item::{ 
    SubmitItem, UpdateItem, ItemID, ItemsPerID, CollectItem, CollectIDs, 
    CollectID, UpdateCollect, DelCollect, StarItem, NewStarItem, StarItemStatus
};
use model::user::{ CheckUser };

pub fn submit_item((item, req, user): (Json<SubmitItem>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // do some check of input
    let l_t = item.title.trim().len();
    if l_t == 0 || l_t > 120 {
        use api::gen_response;
        return gen_response(req)
    }
    
    req.state().db.send( SubmitItem {
        title: item.title.clone(),
        uiid: item.uiid.clone(),
        authors: item.authors.clone(),
        pub_at: item.pub_at.clone(), 
        publisher: item.publisher.clone(),
        category: item.category.clone(),
        url: item.url.clone(),
        cover: item.cover.clone(), 
        edition: item.edition.clone(),
        detail: item.detail.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn get_item(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let item_id = String::from(req.match_info().get("itemid").unwrap());
    req.state().db.send(
        ItemID{item_id}
    )
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn get_item_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let per = req.match_info().get("per").unwrap();  // per tag, user, rut
    let perid = String::from(req.match_info().get("id").unwrap());

    use base64::decode;
    
    let itemsPerID = match per {
        // hope can fuzzy query per uiid..url, contains
        "uiid" => ItemsPerID::Uiid(perid),
        "title" => ItemsPerID::Title(perid),
        "url" => ItemsPerID::ItemUrl(
            String::from_utf8(decode(&perid).unwrap()).unwrap()
        ),
        // query per relations with  rut, tag, user
        "rut" => ItemsPerID::RutID(perid),
        "tag" => ItemsPerID::TagID(perid),
        "user" => ItemsPerID::UserID(
            perid, 
            req.query().get("flag").unwrap().clone(),  // flag 
            req.query().get("page").unwrap().parse::<i32>().unwrap() // paging
        ),
        "key" => ItemsPerID::KeyID(  // hope never use
            req.query().get("keyword").unwrap().clone(), // keyword
            req.query().get("from").unwrap().clone(),  // ?keyword= &from=tag|user
            perid,  // from id
            req.query().get("page").unwrap().parse::<i32>().unwrap() // paging
        ),
        _ => ItemsPerID::ItemID(perid),
    };

    req.state().db.send(itemsPerID)
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn update_item((item, req, user): (Json<UpdateItem>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // do some check of input
    let l_t = item.title.trim().len();
    if l_t == 0 || l_t > 120 {
        use api::gen_response;
        return gen_response(req)
    }
    req.state().db.send( UpdateItem {
        id: item.id.clone(),
        title: item.title.clone(),
        uiid: item.uiid.clone(),
        authors: item.authors.clone(),
        pub_at: item.pub_at.clone(),
        publisher: item.publisher.clone(),
        category: item.category.clone(),
        url: item.url.clone(),
        cover: item.cover.clone(),  
        edition: item.edition.clone(),
        detail: item.detail.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn collect_item((item, req, user): (Json<CollectItem>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // do some check of input
    let l_r = item.rut_id.trim().len();
    let l_i = item.item_id.trim().len();
    if l_r == 0 || l_i == 0 {
        use api::gen_response;
        return gen_response(req)
    }
    
    req.state().db.send( CollectItem {
        rut_id: item.rut_id.clone(),
        item_id: item.item_id.clone(),
        item_order: item.item_order.clone(), // just reserve, no use in msg handler
        content: item.content.clone(),
        uname: user.uname,
    })
    .from_err().and_then(|res| match res {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn get_collect_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let per = req.match_info().get("per").unwrap();
    let id = String::from(req.match_info().get("id").unwrap());

    let paging = if let Some(i) = req.query().get("page") {
        i.parse::<i32>().unwrap()
    } else { 1 };  // if 0, query all

    let collectIDs = match per {
        "item" => CollectIDs::ItemID(id, paging),
        "rut" => CollectIDs::RutID(id),
        "user" => CollectIDs::UserID(id, paging),
        _ => CollectIDs::RutID(id),
    };

    req.state().db.send(collectIDs)
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn get_collect(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let collect_id = String::from(req.match_info().get("cid").unwrap());
    let action = String::from("GET");
    req.state().db.send(
        CollectID{ collect_id, action }
    )
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn del_collect((dc, req, user): (Json<DelCollect>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // do some check
    let c_id = String::from(req.match_info().get("cid").unwrap());
    if c_id != dc.collect_id  || dc.uname != user.uname {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send( DelCollect{
        collect_id: dc.collect_id.clone(),
        rut_id: dc.rut_id.clone(),
        item_id: dc.item_id.clone(),
        uname: user.uname.clone(),   // pass to handler to check permission
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn update_collect((c, req, user): (Json<UpdateCollect>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // need to check the auth_uname == collect_uname, on frontend??
    // check id eque
    let cid = String::from(req.match_info().get("cid").unwrap());
    if cid != c.id || c.uname != user.uname {
        use api::gen_response;
        return gen_response(req)
    }
    req.state().db.send( UpdateCollect {
        id: c.id.clone(),
        content: c.content.clone(),
        uname: user.uname.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn star_item(req: HttpRequest<AppState>, user: CheckUser)
 -> FutureResponse<HttpResponse> {
    let flag = String::from(req.match_info().get("flag").unwrap()); // no need?
    let itemid = String::from(req.match_info().get("itemid").unwrap());
    let note = String::from(req.match_info().get("note").unwrap());
    
    req.state().db.send( NewStarItem {
        uname: user.uname.clone(),
        item_id: itemid.clone(),
        note: note.clone(),
        flag: flag.clone(), // not use in msg handler, just reserve
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn star_item_status(req: HttpRequest<AppState>, user: CheckUser)
 -> FutureResponse<HttpResponse> {
    let uname = user.uname;
    let item_id = String::from(req.match_info().get("itemid").unwrap());
    
    req.state().db.send( 
        StarItemStatus { uname, item_id }
    )
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}
