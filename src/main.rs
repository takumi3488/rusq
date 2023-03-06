use std::io;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use actix_web_httpauth::{
    extractors::{
        basic::{self, BasicAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use utils::config::get_config;
mod routes;
mod utils;
use crate::routes::{create_queue, get_first_queue, get_queues};

async fn basic_auth_validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let (_, _, user, password, _) = get_config();
    if credentials.user_id() == &user && credentials.password() == Some(&password) {
        Ok(req)
    } else {
        let config = req.app_data::<basic::Config>().cloned().unwrap_or_default();
        Err((AuthenticationError::from(config).into(), req))
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let (port, host, _, _, redis_url) = get_config();
    let redis_client = redis::Client::open(redis_url).unwrap();

    println!("RusQ started with http://{}:{}", host, port);

    // Start RusQ
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_client.clone()))
            .service(get_queues)
            .service(create_queue)
            .service(get_first_queue)
            .wrap(HttpAuthentication::basic(basic_auth_validator))
    })
    .bind((host, port))?
    .run()
    .await
}
