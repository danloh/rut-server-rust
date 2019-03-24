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
use regex::Regex;
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

// re test
// for re test uname
pub fn re_test_uname(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[\w-]{3,42}$").unwrap();
    }
    RE.is_match(text)
}

// for re test url
pub fn re_test_url(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(https?):\/\/([^/:]+)(:[0-9]+)?(\/.*)?$").unwrap();
    }
    RE.is_match(text)
}

pub fn len_limit(text: &str, min: usize, max: usize) -> bool {
    let l = text.len();
    l >= min && l <= max
}
