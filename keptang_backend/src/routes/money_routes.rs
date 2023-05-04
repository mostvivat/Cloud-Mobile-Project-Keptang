use actix_web::web::{self};
use crate::handlers::money_handler::* ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_money);

        
}