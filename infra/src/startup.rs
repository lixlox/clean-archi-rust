use crate::state::state_factory;
use crate::web::routes::books::get_all;
use crate::web::routes::health_check::health_check;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .data_factory(state_factory)
            .route("/health_check", web::get().to(health_check))
            .route("/book", web::get().to(get_all))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
