use serde::{Serialize, Deserialize};
use crate::config::db::con_db;
use mysql::*;
use mysql::prelude::*;

// request user
#[derive(Serialize, Deserialize)]
pub struct AddRequest1 {
    pub user_id: i32
}
// request description,date,amount,types
#[derive(Serialize, Deserialize)]
pub struct AddRequest2 {
    pub description: String,
    pub date: String,
    pub amount: i32,
    pub types: String
}

// request all combine
#[derive(Serialize, Deserialize)]
pub struct AddRequest {
    pub user_data: AddRequest1,
    pub data_item: AddRequest2
}

//  add-return-database
pub fn insert_money(user_id:i32 , description:String,date:String,amount:i32,types:String){
    let _ = match con_db() {
        Ok(mut conn) => {
            conn.exec_drop(
            "INSERT INTO moneylist ( description, date, amount, types, user_id) 
            VALUES ( :description , :date, :amount , :types, :user_id );",
            params! {
                "user_id" => user_id,
                "description" => description,
                "date" => date,
                "amount" =>amount,
                "types" =>types
            },
        )},
        Err(e) => {
            println!("Failed to get DB connection: {}", e);
            return;
        }
    };
}



    