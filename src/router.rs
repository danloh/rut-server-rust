#![allow(warnings)]
#![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use actix_web::{
    App, actix::Addr, fs,
    middleware::{self, cors::Cors},
};
use db::dba::{ Dba, init };
use api::rut::{ new_rut, get_rut, get_rut_list };

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
        .resource("/home", |r| {})
        .resource("/ruts", |r| {
            // r.get().f();
            r.post().with(new_rut);
        })
        .resource("/ruts/{rid}", |r| {
            r.get().with(get_rut);
        })
        .resource("/ruts/{type:[0|1|2]}/{tid}", |r| { // Type: 0- user, 1- item, 2- index
            r.get().with(get_rut_list);
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
