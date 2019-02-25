#![allow(warnings)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use actix_web::{
    App, actix::Addr, fs,
    middleware::{self, cors::Cors},
};
use db::dba::{ Dba, init };
use api::auth::{ signup, signin, check_user, auth_token };
use api::rut::{ new_rut, get_rut, get_rut_list, update_rut, star_unstar_rut };
use api::item::{ submit_item, get_item, get_item_list, update_item, collect_item };

pub struct AppState {
    pub db: Addr<Dba>,
}

pub fn app_with_state() -> App<AppState> {
    App::with_state(AppState{ db: init().clone()})
    // enable logger
    .middleware(middleware::Logger::default())
    // enable cors 
    .middleware(Cors::default())
    // config resource, router, REST-style, under '/api/'
    .scope("/api", |api| { api
        .resource("/home", |r| {})
        .resource("/signup", |r| { 
            r.post().with(signup); 
        })
        .resource("/checkuser/{uname}", |r| { 
            r.get().with(check_user); 
        })
        .resource("/signin", |r| { 
            r.post().with(signin);
            r.get().with(auth_token); 
        })
        .resource("/ruts", |r| {
            r.post().with(new_rut);
        })
        .resource("/ruts/{rid}", |r| {
            r.get().with(get_rut);
            r.post().with(update_rut);
        })
        .resource("/ruts/{type:[0|1|2]}/{tid}", |r| { // Type: 0- user, 1- item, 2- index
            r.get().with(get_rut_list);
        })
        .resource("/ruts/{rid}/{action:[0|1]}/star", |r| { // 0- unstar, 1- star
            r.get().with(star_unstar_rut);
        })
        .resource("/ruts/{rid}/collect", |r| {
            r.post().with(collect_item);
        })
        .resource("/items", |r| {
            r.post().with(submit_item);
        })
        .resource("/items/{itemid}", |r| {
            r.get().with(get_item);
            r.post().with(update_item);
        })
        .resource("/items/{per}/{itemid}", |r| {
            r.get().with(get_item_list);
        })
    })
    // or: /* .prefix("/api").configure( |app| { Cors::for_app(app).max_age(3600) }) */
    // handle static files
    .handler("/static", fs::StaticFiles::new("./static/").unwrap().index_file("index.html"))
    // redirect
    .resource("/", |r| { /* todo: redirect */ })
    // default
    .default_resource(|r| { /* todo: default, for 404, etc. */ })
}
