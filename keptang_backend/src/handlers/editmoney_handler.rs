use actix_web::{put, web, HttpResponse};
use log::{debug, info};
use crate::models::editmoney_model::*;
use crate::models::money_model::*;

// PUT /money/edit/{id}: รับ JSON ที่มีค่าของคีย์ "expense"/"income" เป็นรายการ JSON ที่มีข้อมูลของรายการรายจ่ายที่ต้องการอัปเดตด้วย ID ที่ระบุ
#[put("/money/item/{id}")]
async fn put_money(list_id: web::Path<i32>,input_data: web::Json<EditRequest>) -> HttpResponse {
    info!("put money by id");
    debug!("id: {} 🪄", list_id);

    // ค่าเริ่มต้น ที่รับมาแบบ JSON (ถ้าอยากแก้ไข เติม mut หลัง let)
    let user_data = input_data.into_inner();
    let id: i32 = list_id.to_string().parse().unwrap();
    let types_edit = user_data.data_item.types;
    let types_new = types_edit.clone();
    
    
    let mut types_old:String = "".to_string();
    let mut amount_old = 0;
    let amount_new = user_data.data_item.amount;

    // ดึงค่าเก่ามาเช็คเพื่อคืนค่า amount
    let data = get_moneylist_byid(user_data.UserData.user_id,id);
        for i in data {
            types_old = i.types;
            amount_old = i.amount;
        }
    
    // กลับคั่ว type เพราะเราลบค่าออกไป
    if types_old.to_string() == "income"{
        types_old = "expenses".to_string();
    }else{
        types_old = "income".to_string();
    }
    // debug!("คืนค่าเดิมก่อน 🪄");
    // debug!("amount_old: {} 🪄", amount_old);
    // debug!("types: {} 🪄", types_old);
    // แก้ไข ยอดเงินทั้งหมดเป็นยอดเดิม
    edit_balance_total(user_data.UserData.user_id,amount_old,types_old);

    // แก้ไขค่าใหม่
    edit_money(user_data.UserData.user_id,
        id,
        user_data.data_item.description,
        user_data.data_item.date,
        user_data.data_item.amount,
        types_edit);


    // debug!("แก้ค่าใหม่ 🪄");
    // debug!("amount_old: {} 🪄", amount_new);
    // debug!("types: {} 🪄", types_new);

    // แก้ไข ยอดเงินทั้งหมดเป็นค่าล่าสุด
    edit_balance_total(user_data.UserData.user_id,amount_new,types_new);

    HttpResponse::Ok().body("ทำการแก้ไขข้อมูลสำเร็จ👌")
}