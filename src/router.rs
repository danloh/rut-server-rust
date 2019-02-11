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
use api::index::{ hello };
use api::rut::{ new_rut };

pub struct AppState {
    pub db: Addr<Dba>,
}

pub fn app_with_state() -> App<AppState> {
    App::with_state(AppState{ db: init().clone()})
    // enable logger
    .middleware(middleware::Logger::default())
    .prefix("/api")
    // config resource, router, REST-style 
    .configure( |app| Cors::for_app(app)
        .max_age(3600)
        .resource("/ruts/index", |r| {r.get().with(hello)})  // to be del
        // register routes
        .resource("/home", |r| {})
        .resource("/ruts", |r| {
            // r.get().f();
            r.post().with(new_rut);
        })
        .register()
    )
    // static files
    .handler("/static", fs::StaticFiles::new("static").unwrap())
    // redirect
    .resource("/", |r| { /* todo: redirect */ })
    // default
    .default_resource(|r| { /* todo: default, for 404, etc. */ })
}
