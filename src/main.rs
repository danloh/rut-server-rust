#![allow(warnings)]
#![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate uuid;
extern crate dotenv;
extern crate chrono;
extern crate num_cpus;
extern crate bcrypt;

use actix_web::{ server, actix::System };
use std::{ env };

mod router;
mod db;
mod api;

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
