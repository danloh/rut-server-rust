// api.index

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    Error, Json, State, HttpMessage, Result, http::StatusCode 
};
use futures::Future;
use router::AppState;
use model::user::CreateUser;


// just for test
#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    cover: String,
    createat: String,
    id: i32,
    intro: String,
    itemcount: i32,
    title: String
}

#[derive(Debug, Serialize, Deserialize)]
struct RutObj {
    ruts: Vec<MyObj>
}

pub fn hello(req: HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("{:?}", req);

    Ok(HttpResponse::build(StatusCode::OK)
    .content_type("application/json; charset=utf-8")
    .json( RutObj {
        ruts: vec!( MyObj {
                cover: String::from("https://camo.githubusercontent.com/d2e5827a412359c7593575adf876db23d4d50747/68747470733a2f2f692e696d6775722e636f6d2f6259776c3756662e706e67"), 
                createat: String::from("2017-07-08 03:11:45"),
                id: 42,
                intro: String::from("<p>Ancient Greece: with emphasis on the Golden Age of Greece. Non-fiction but can also include ancient Greek literature, myths, poetry, drama and philosophy.</p>"), 
                itemcount: 6,
                title: String::from("Best Books on Ancient Greek History and Literature"), 
            })
        }))  // <- send response
    // Ok(HttpResponse::build(StatusCode::OK)
       // .content_type("text/html; charset=utf-8")
        // .json(MyObj {name: String::from("Tom"), number: 42})) // MyObj {name: "Tom", number: 42}
}
