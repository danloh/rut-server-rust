#![allow(warnings)]
// #![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;

// eliminate magic number
const PER_PAGE: i32 = 20;   // for paging
const ANS_LIMIT: usize = 42;  // limit tag len, fo tags, collect item
const MAX_UNAME_LEN: usize = 16;
const MIN_PSW_LEN: usize = 8;
const MIN_LEN: usize = 1;
const INPUT_LIMIT: usize = 512;  // limit input title, url

use actix::{ Actor, SyncContext };
use actix::prelude::*;
use actix_web::{
    web::{ self, scope, resource, route, get, post, put, delete },
    middleware::{ Logger, cors::Cors },
    App, HttpServer, HttpResponse
};

use diesel::pg::PgConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use dotenv::dotenv;

// #[macro_use]
// pub mod macros;

mod db;
mod api;
mod model;
mod util;
mod errors;
mod schema;

// This is db executor actor
pub struct Dba(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for Dba {
    type Context = SyncContext<Self>;
}

pub type DbAddr = Addr<Dba>;

pub fn init_dba() -> DbAddr {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let cpu_num = num_cpus::get();
    let pool_num = (cpu_num) as u32;
    // p_num subject to c_num?? 
    let conn = Pool::builder()
        .max_size(pool_num)
        .build(manager)
        .expect("Failed to create pool.");

    SyncArbiter::start( 
        cpu_num * 2, 
        move || { Dba(conn.clone()) }
    )
}


fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "rut-server-rust=debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let sys = actix::System::new("rut-server-rust");
    let addr: DbAddr = init_dba();

    let bind_host = dotenv::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set");

    HttpServer::new( move || { App::new()
        .data(addr.clone())
        .wrap(Logger::default())
        .wrap(Cors::default())
        // everything under '/api/' route
        .service(scope("/api")
            // to auth
            .service(resource("/signin")
                .route(post().to_async(api::auth::signin))
            )
            // to register 
            .service(resource("/signup")
                .route(post().to_async(api::auth::signup))
            )
            // get / update user, change password
            .service(resource("/users/{uname}")
                .route(get().to_async(api::auth::get))
                .route(post().to_async(api::auth::update))
                .route(put().to_async(api::auth::change_psw))
            )
            .service(resource("/ruts")
                .route(post().to_async(api::rut::new))
                .route(put().to_async(api::rut::update))
            )
            .service(resource("/ruts/{slug}")
                .route(get().to_async(api::rut::get))
                .route(post().to_async(api::rut::update))  // per frontend, can be del
                //.route(delete().to_async(api::rut::delete))
            )
            .service(resource("/ruts/{per}/{perid}")  // ?page=p&flag=create|star&kw= fr=
                .route(get().to_async(api::rut::get_list))
            )
            .service(resource("/starrut/{rutid}/{action:[0|1]}/{note}")
                .route(get().to_async(api::rut::star_or_unstar))
            )
            .service(resource("/ifstarrut/{rutid}")
                .route(get().to_async(api::rut::star_status))
            )
            .service(resource("/items")
                .route(post().to_async(api::item::new))
                .route(put().to_async(api::item::update))
            )
            .service(resource("/items/{slug}")
                .route(get().to_async(api::item::get))
                .route(post().to_async(api::item::update))  // can be del, per frontend
                // .route(delete().to_async(api::item::delete))
            )
            .service(resource("/items/{per}/{id}")
                .route(get().to_async(api::item::get_list))
            )
            .service(resource("/staritem/{itemid}/{flag:[1|2|3]}/{rate}/{note}")
                .route(get().to_async(api::item::star_item))
            )
            .service(resource("/itemflag/{itemid}")
                .route(get().to_async(api::item::star_status))
            )
            .service(resource("/collectitem/{rutid}")
                .route(post().to_async(api::item::collect_item))
            )
            .service(resource("/collects/{per}/{id}")
                .route(get().to_async(api::item::get_collect_list))
            )
            .service(resource("/collects/{cid}")
                .route(get().to_async(api::item::get_collect))
                .route(put().to_async(api::item::update_collect))
                .route(delete().to_async(api::item::del_collect))
            )
            .service(resource("/tags/{tname}")
                .route(get().to_async(api::tag::get))
                .route(put().to_async(api::tag::update))
                .route(post().to_async(api::tag::new))
            )
            .service(resource("/tags/{per}/{id}")
                .route(get().to_async(api::tag::get_list))
            )
            .service(resource("/tagr/{action:[0|1]}/{rutid}")
                .route(post().to_async(api::tag::tag_rut))
            )
            .service(resource("/startag/{tname}/{action:[0|1]}/{note}")
                .route(get().to_async(api::tag::star_or_unstar))
            )
            .service(resource("/ifstartag/{tname}")
                .route(get().to_async(api::tag::star_status))
            )
            .service(resource("/etcs")
                .route(post().to_async(api::etc::new))
            )
            .service(resource("/etcs/{per}/{perid}")
                .route(post().to_async(api::etc::get_list))
            )
            .default_service(
                route().to(|| HttpResponse::NotFound())
            )
        )
    })
    .bind(&bind_host).expect("Can not bind to host")
    .start();

    println!("Starting http server: {}", bind_host);
    
    // start runtime
    sys.run()
}
