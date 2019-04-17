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
use api::{ re_test_url, len_limit };
use ::INPUT_LIMIT;

pub fn submit_item((item, req, user): (Json<SubmitItem>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // do some check of input
    // check url and cover img url
    // required: title, author
    let url = item.url.trim();
    let cover = item.cover.trim();
    let url_test = if url.len() == 0 { true } else { re_test_url(url) };
    let cover_test = if cover.len() == 0 { true } else { re_test_url(cover) };
    let title = item.title.trim();
    let uiid = item.uiid.trim();
    let authors = item.authors.trim();
    let pub_at = item.pub_at.trim();
    let publisher = item.publisher.trim();
    let category = item.category.trim();
    let edition = item.edition.trim();
    let check_len = len_limit(title, 1, INPUT_LIMIT)
        && len_limit(uiid, 0, 32) && len_limit(authors, 1, 64) 
        && len_limit(pub_at, 0, 32) && len_limit(publisher, 0, 64) 
        && len_limit(category, 0, 32) && len_limit(edition, 0, 64);

    if !check_len || !url_test || !cover_test {
        use api::gen_response;
        return gen_response(req)
    }
    
    req.state().db.send( SubmitItem {
        title: title.to_string(),
        uiid: uiid.to_string(),
        authors: authors.to_string(),
        pub_at: pub_at.to_string(), 
        publisher: publisher.to_string(),
        category: category.to_string(),
        url: url.to_string(),
        cover: cover.to_string(),
        edition: edition.to_string(),
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
    
    use base64::decode;  // for decode url

    let paging = if let Some(i) = req.query().get("page") {
        i.parse::<i32>().unwrap()
    } else { 1 };  // if 0, query all
    
    let itemsPerID = match per {
        // hope can fuzzy query per uiid..url, contains
        // here are some issue, 400 or no result, % trimed
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
            paging,
        ),
        "key" => ItemsPerID::KeyID(  // hope never use
            req.query().get("keyword").unwrap().clone(), // keyword
            req.query().get("from").unwrap().clone(),  // ?keyword= &from=tag|user
            perid,  // from id
            paging,
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
    // check url and cover img url
    // required: id, title, author
    let itemid = item.id.trim();
    let url = item.url.trim();
    let cover = item.cover.trim();
    let url_test = if url.len() == 0 { true } else { re_test_url(url) };
    let cover_test = if cover.len() == 0 { true } else { re_test_url(cover) };
    let title = item.title.trim();
    let uiid = item.uiid.trim();
    let authors = item.authors.trim();
    let pub_at = item.pub_at.trim();
    let publisher = item.publisher.trim();
    let category = item.category.trim();
    let edition = item.edition.trim();
    let check_len = len_limit(itemid, 8, INPUT_LIMIT)
        && len_limit(title, 1, INPUT_LIMIT)
        && len_limit(uiid, 0, 32) && len_limit(authors, 1, 64) 
        && len_limit(pub_at, 0, 32) && len_limit(publisher, 0, 64) 
        && len_limit(category, 0, 32) && len_limit(edition, 0, 64);

    if !check_len || !url_test || !cover_test {
        use api::gen_response;
        return gen_response(req)
    }

    req.state().db.send( UpdateItem {
        id: itemid.to_string(),
        title: title.to_string(),
        uiid: uiid.to_string(),
        authors: authors.to_string(),
        pub_at: pub_at.to_string(), 
        publisher: publisher.to_string(),
        category: category.to_string(),
        url: url.to_string(),
        cover: cover.to_string(),
        edition: edition.to_string(),
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

pub fn del_collect((req, user): (HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    // should do some check in frontend

    let c_id = String::from(req.match_info().get("cid").unwrap());

    req.state().db.send( DelCollect{
        collect_id: c_id,
        uname: user.uname,   // pass to handler to check permission
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
    let rate: i32 = req.match_info().get("rate").unwrap().parse().unwrap();
    let note = String::from(req.match_info().get("note").unwrap());

    // flag only can be todo, doing, done
    let flg = flag.to_lowercase();
    if flg != "todo" && flg != "doing" && flg != "done" {
        use api::gen_response;
        return gen_response(req)
    }
    
    req.state().db.send( NewStarItem {
        uname: user.uname.clone(),
        item_id: itemid.clone(),
        note: note.clone(),
        flag: flag.clone(),
        rate: rate.clone(),
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
