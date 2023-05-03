use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
// เป็น object ที่เราสร้างขึ้นมา  ตารางเก็บ รายรับ-รายจ่าย
pub struct Moneylist {
    pub list_id: i32,
    pub description: String,
    pub date: String,
    pub amount: i32,
    pub types: String,
}
// ตารางเก็บ บัญชีผู้ใช้
#[derive(Serialize, Deserialize)]
pub struct Userdata {
    pub id: i32,
    pub name: String,
    pub balance_total: i32,
}
