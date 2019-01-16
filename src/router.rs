#![allow(warnings)]
#![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use bytes::Bytes;
use futures::sync::mpsc;
use futures::Stream;

use actix_web::{
    error, fs, middleware, pred, server, App, Error, 
    HttpRequest, HttpResponse, Path, Result,
    http::{header, Method, StatusCode},
};
use std::{io};

/// simple index handler
fn index(req: &HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

/// 404 handler
fn not_found(req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

pub fn app_state() -> App {
    App::new()
    // enable logger
    .middleware(middleware::Logger::default())        
    // register simple route, handle all methods
    .resource("/", |r| r.f(index))
    // static files
    .handler("/static", fs::StaticFiles::new("static").unwrap())
    // redirect
    .resource("/", |r| r.method(Method::GET).f(|req| {
        println!("{:?}", req);
        HttpResponse::Found()
            .header(header::LOCATION, "static/index.html")
            .finish()
    }))
    // default
    .default_resource(|r| {
        // 404 for GET request
        r.method(Method::GET).f(not_found);

        // all requests that are not `GET`
        r.route().filter(pred::Not(pred::Get()))
         .f(|req| HttpResponse::MethodNotAllowed());
    })
}
