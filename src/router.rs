#![allow(warnings)]
#![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use actix_web::{
    App, fs, pred, actix::Addr,
    HttpRequest, HttpResponse, Result,
    http::{self, header, Method, StatusCode},
    middleware::{self, cors::Cors},
};
use db::dba::{ Dba, init };
use api::{index::hello};

pub struct AppState {
    pub db: Addr<Dba>,
}

pub fn app_with_state() -> App<AppState> {
    App::with_state(AppState{ db: init().clone()})
    // enable logger
    .middleware(middleware::Logger::default())        
    // register simple route, handle all methods
    .resource("/", |r| r.f(index))
    .configure( |app| Cors::for_app(app)
        .max_age(3600)
        // register routes
        .resource("/api/home", |r| {})
        .register()
    )
    // static files
    .handler("/static", fs::StaticFiles::new("static").unwrap())
    // redirect
    .resource("/", |r| { /* todo: redirect */ })
    // default
    .default_resource(|r| { /* todo: default, for 404, etc. */ })
}


// index handler, just for try
fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("{:?}", req);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}
