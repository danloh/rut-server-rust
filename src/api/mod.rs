// api mod

// view handler, to handle the request from client,
// to be simply, 
// send msg to Db Actor, then handled by msg-handler of Actor
// recv the result msg from msg-handler, response to request

// model: build msg and traited-table struct;  
// view handler: handle request - send msg - response;  
// msg handler: handle msg, comm with db, send back result;  
// actor: db, s-r-h;

pub mod index;
pub mod auth;
pub mod rut;
pub mod item;
pub mod tag;
pub mod etc;

use actix_web::{HttpResponse,HttpRequest,HttpMessage,FutureResponse,AsyncResponder};
use futures::Future;
use router::AppState;
use model::msg::Msg;

// build response if anything wrong in checking req before send msg, 
// need to optmize, alert: some issue, no real resp, just bad request error
// how to new a Future directly?
pub fn gen_response(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.json().from_err().and_then(|res: Msg| { // maybe Type notation the issue?
        Ok(HttpResponse::Ok().json(
            Msg {status: 422, message:"Unprocessable".to_string()}
        ))
    })
    .responder()
}


// Note: new, update, post, need to auth the identity, i.e. who can update
// auth in frontend or backend or both ?? -- to do
