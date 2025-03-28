use actix_web::web;

use super::controller::create;

pub fn config(conf: &mut web::ServiceConfig) {
    let routes = web::scope("/administrator")
        .service(create);
    conf.service(routes);
}