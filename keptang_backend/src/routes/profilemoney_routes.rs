use actix_web::web::{self};
use crate::handlers::profilemoney_handler::* ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_profile);

        
}