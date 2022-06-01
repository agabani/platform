use axum::{routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;

pub fn create(router: Router) -> Server<AddrIncoming, IntoMakeService<Router>> {
    Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(router.into_make_service())
}
