#![allow(warnings)]
// #![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

extern crate futures;
extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use] 
extern crate serde_derive;
#[macro_use] 
extern crate diesel;
extern crate uuid;
extern crate chrono;
extern crate num_cpus;
extern crate bcrypt;
extern crate jsonwebtoken as jwt;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate base64;

use actix_web::{ server, actix::System };
use std::{ env };

mod router;
mod db;
mod api;
mod model;
mod handler;
mod util;

// eliminate magic number
const PER_PAGE: i32 = 20;   // for paging
const ANS_LIMIT: usize = 42;  // limit tag len, fo tags, collect item
const MAX_UNAME_LEN: usize = 16;
const MIN_PSW_LEN: usize = 8;
const MIN_LEN: usize = 1;


fn main() {
    env::set_var("RUST_LOG", "rut-server-rust=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let sys = actix::System::new("rut-server-rust");

    server::new( move || router::app_with_state())
        .bind("127.0.0.1:8083").expect("Can not bind to 127.0.0.1:8083")
        .shutdown_timeout(0)    // <- Set shutdown timeout to 0 seconds (default 60s)
        .start();
    println!("Starting http server: 127.0.0.1:8083");
    
    let _ = sys.run();
}
