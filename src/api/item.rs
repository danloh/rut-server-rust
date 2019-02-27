// api.item, view handler

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State
};
use futures::Future;
use router::AppState;
use model::item::{ 
    SubmitItem, UpdateItem, ItemID, ItemIDs, ItemsPerID, CollectItem, CollectID 
};
use model::user::{ CheckUser };

pub fn submit_item((item, state, user): (Json<SubmitItem>, State<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    state.db.send( SubmitItem {
        title: item.title.clone(),
        uiid: item.uiid.clone(),
        pub_at: item.pub_at.clone(), 
        authors: item.authors.clone(),
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
    let per = req.match_info().get("per").unwrap();
    let item_id = String::from(req.match_info().get("itemid").unwrap());
    let itemIDs = match per {
        "id" => ItemIDs::ID(item_id),
        "uiid" => ItemIDs::Uiid(item_id),
        "title" => ItemIDs::Title(item_id),
        "url" => ItemIDs::Url(item_id),
        _ => ItemIDs::ID(item_id),
    };

    req.state().db.send(itemIDs)
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn get_items_per(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let per = req.match_info().get("per").unwrap();  // per tag, user, rut
    let perid = String::from(req.match_info().get("id").unwrap());
    let flag = String::from(req.match_info().get("flag").unwrap());
    let itemsPerID = match per {
        "rut" => ItemsPerID::RutID(perid),
        "tag" => ItemsPerID::TagID(perid),
        // "user" => ItemsPerID::UserID(perid, flag),
        _ => ItemsPerID::RutID(perid),
    };

    req.state().db.send(itemsPerID)
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn update_item((item, state, user): (Json<UpdateItem>, State<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    state.db.send( UpdateItem {
        id: item.id.clone(),
        title: item.title.clone(),
        uiid: item.uiid.clone(),
        pub_at: item.pub_at.clone(),
        authors: item.authors.clone(),
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

pub fn collect_item((item, state, user): (Json<CollectItem>, State<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    state.db.send( CollectItem {
        rut_id: item.rut_id.clone(),
        item_id: item.item_id.clone(),
        item_order: item.item_order.clone(),  
        content: item.content.clone(), 
        creator_id: user.id,
    })
    .from_err().and_then(|res| match res {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

pub fn get_collect(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let rut_id = String::from(req.match_info().get("rutid").unwrap());
    let item_id = String::from(req.match_info().get("itemid").unwrap());
    req.state().db.send(
        CollectID{rut_id, item_id}
    )
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}
