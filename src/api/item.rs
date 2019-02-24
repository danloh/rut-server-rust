// api.item, view handler

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State
};
use futures::Future;
use router::AppState;
use model::item::{ SubmitItem, ItemID, ItemIDs, CollectItem };
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

