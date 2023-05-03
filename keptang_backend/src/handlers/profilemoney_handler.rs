use actix_web::{ get, HttpResponse, Responder, web};
use log::{info};
use crate::models::profile_model::*;



// GET /profile 
#[get("/profile/{id}")]
async fn get_profile(user_id: web::Path<i32>) -> impl Responder {
    info!("Keptang profile");
    let id: i32 = user_id.to_string().parse().unwrap();
    // let userdata = user_id.into_inner();
    // let id: i32 = userdata.id;  
    //ฟังก์ชันget_user_profile จาก src\models\moneylist.rs
    let data = get_user_profile(id);

   HttpResponse::Ok().json(data)
}