use actix_web::{web, HttpResponse, Responder,post};
use log::{debug, info};
use crate::models::addmoney_model::*;
use crate::models::editmoney_model::*;


// POST /money: สำหรับเพิ่มข้อมูลรายการรายรับรายจ่ายใหม่
#[post("/money/saving")]
async fn post_money(input_data: web::Json<AddRequest>) -> impl Responder {
    info!("post money");
    debug!("post: ✅");

    // ค่าเริ่มต้น ที่รับมาแบบ JSON (ถ้าอยากแก้ไข เติม mut หลัง let)
    let user_data = input_data.into_inner();
    let types = user_data.data_item.types;
    let types2 = types.clone();
    // function addmoney
    insert_money(user_data.user_data.user_id,
        user_data.data_item.description,
        user_data.data_item.date,
        user_data.data_item.amount,
        types);

    // แก้ไข ยอดเงินทั้งหมดด้วย
    edit_balance_total(user_data.user_data.user_id,user_data.data_item.amount,types2);

    // HttpResponse::Ok().json(response_body)   ถ้าตัวนี้จะเป็น Status Code 200
    HttpResponse::Created().body("ทำการเพิ่มข้อมูลสำเร็จ👌") 

}
