// api.item, view handler

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State
};
use futures::Future;
use router::AppState;
use model::item::{ SubmitItem, ItemID };
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
