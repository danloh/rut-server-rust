// api mod

// view handler, to handle the request from client,
// to be simply,
// send msg to Db Actor, then handled by msg-handler of Actor
// recv the result msg from msg-handler, response to request

// actor: db, typed model,  msg handler

pub mod auth;
pub mod etc;
pub mod item;
pub mod rut;
pub mod tag;

// for extract typed request Query info: /path?page=&flag=&kw=&fr=
#[derive(Deserialize, Clone)]
pub struct ReqQuery {
    page: i32,
    flag: String,
    kw: String, // keyword  Option<String>?
    fr: String, // from user|tag..
}
