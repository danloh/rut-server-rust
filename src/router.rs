#![allow(warnings)]
#![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use actix_web::{
    App, fs, middleware, pred, actix::Addr,
    HttpRequest, HttpResponse, Result,
    http::{self, header, Method, StatusCode},
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
    .resource("/{name}", |r| r.method(http::Method::GET).with(hello))
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

// some simple handler, index, 404...
// index handler
fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("{:?}", req);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

// 404 handler
fn not_found(req: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}
