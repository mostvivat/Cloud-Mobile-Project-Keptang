use actix_web::web::{self};
use crate::handlers::deletemoney_handler::* ;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(delete_money);
}