#![allow(warnings)]
#![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use actix_web::{ server, actix::System };
use std::{ env };

mod router;

fn main() {
    env::set_var("RUST_LOG", "rut-server-rust=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let sys = actix::System::new("rut-server-rust");

    server::new( move || router::app_state())
        .bind("127.0.0.1:8083").expect("Can not bind to 127.0.0.1:8083")
        .shutdown_timeout(0)    // <- Set shutdown timeout to 0 seconds (default 60s)
        .start();
    println!("Starting http server: 127.0.0.1:8083");
    
    let _ = sys.run();
}
