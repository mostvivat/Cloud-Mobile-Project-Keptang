use actix_web::{delete, web, HttpResponse};
use log::{debug, info};
use crate::models::deletemoney_model::*;
use crate::models::editmoney_model::*;
use crate::models::money_model::*;


// DELETE /money/delete/{id}: ไม่ต้องการรับข้อมูลใดๆ แต่จะลบรายการรายจ่ายที่มีอยู่ด้วย list_id ที่ระบุ
#[delete("/money/item/{id}")]
async fn delete_money(list_id: web::Path<i32>,input_data: web::Json<DeleteRequest>) -> HttpResponse {
    info!("delete money by list_id");
    debug!("list_id: {} ❌", list_id);

    // ค่าเริ่มต้น ที่รับมาแบบ JSON (ถ้าอยากแก้ไข เติม mut หลัง let)
    let user_id = input_data.into_inner();
    let id: i32 = list_id.to_string().parse().unwrap();
    let mut types:String = "".to_string();
    let mut amount = 0;

    let data = get_moneylist_byid(user_id.user_id,id);
    for i in data {
        types = i.types;
        amount = i.amount;
    }
    // debug!("------------------------------------------ ");
    // debug!("types: {} ", types);
    // debug!("amount: {} ", amount);
    
    // กลับคั่ว type เพราะเราลบค่าออกไป
    if types.to_string() == "income"{
        types = "expenses".to_string();
    }else{
        types = "income".to_string();
    }
    edit_balance_total(user_id.user_id,amount,types);
    delete_money_db(user_id.user_id,id);


    HttpResponse::Ok().body("ทำการลบข้อมูลสำเร็จ👌")
}
