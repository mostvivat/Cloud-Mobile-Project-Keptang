use actix_web::web::{self};
use crate::handlers::addmoney_handler::* ;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(post_money);   
}