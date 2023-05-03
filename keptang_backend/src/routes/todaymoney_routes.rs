use actix_web::web::{self};
use crate::handlers::todaymoney_handler::* ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_money_today);

        
}