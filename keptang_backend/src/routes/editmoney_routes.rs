use actix_web::web::{self};
use crate::handlers::editmoney_handler::* ;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(put_money);
}