#![allow(warnings)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use actix_web::{
    App, actix::Addr, fs,
    middleware::{self, cors::Cors},
};
use db::dba::{ Dba, init };
use api::auth::{ signup, signin, check_user, auth_token, get_user, update_user };
use api::rut::{ new_rut, get_rut, get_rut_list, update_rut, star_unstar_rut };
use api::item::{ 
    submit_item, get_item, get_item_list, 
    update_item, collect_item, get_collect
};
use api::tag::{ new_tag, get_tag, get_tag_list, update_tag, tag_rut };

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
        .resource("/users/{userid}", |r| {
            r.get().with(get_user);
            r.post().with(update_user);
        })
        .resource("/ruts", |r| {
            r.post().with(new_rut);
        })
        .resource("/ruts/{rid}", |r| {
            r.get().with(get_rut);
            r.post().with(update_rut);
        })
        .resource("/ruts/{per}/{tid}/{flag}", |r| { // Per: user,item,tag,index
            r.get().with(get_rut_list);             // flag: create, star
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
        .resource("/items/{per}/{id}/{flag:[0|1|2]}", |r| { // per: rut,tag,user,id,url,title
            r.get().with(get_item_list);                    // flag: 0-to,1-ing,2-done
        })
        .resource("/{rutid}/collects/{itemid}", |r| {
            r.get().with(get_collect);
        })
        .resource("/tags/{tname}", |r| {
            r.get().with(get_tag);
            r.post().with(new_tag);
            r.put().with(update_tag);
        })
        .resource("/tags/{per}/{id}", |r| { // per: rut,tag,user,item
            r.get().with(get_tag_list);
        })
        .resource("/tag/{action:[0|1]}/{rutid}", |r| { // 0-untag,1-tag
            r.post().with(tag_rut);
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
