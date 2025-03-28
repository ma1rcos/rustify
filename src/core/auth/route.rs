use actix_web::web;

use super::controller::auth_administrator;

pub fn config(conf: &mut web::ServiceConfig) {
    let routes = web::scope("/auth")
        .service(auth_administrator);
    conf.service(routes);
}