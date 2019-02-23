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
