use serde::{Serialize, Deserialize};
use crate::config::db::con_db;
use mysql::*;
use mysql::prelude::*;
use crate::models::money_model::*;
//use log::{debug};

//request1edit
#[derive(Serialize, Deserialize)]
pub struct EditRequest1 {
    pub user_id: i32
}

// request2edit
#[derive(Serialize, Deserialize)]
pub struct EditRequest2 {
    pub description: String,
    pub date: String,
    pub amount: i32,
    pub types: String
}


// combine_request
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EditRequest {
    pub UserData: EditRequest1,
    pub data_item: EditRequest2
}

// response
#[derive(Serialize, Deserialize)]
struct EditResponse {
    items_old: MoneyList,
    items_new: MoneyList,
    text: String,
}


//  edit-return-database
pub fn edit_money(user_id:i32 ,list_id:i32, description:String,date:String,amount:i32,types:String){
    let _ = match con_db() {
        Ok(mut conn) => {
            conn.exec_drop(
            "UPDATE `moneylist` SET `description`= :description , `date`= :date, `amount`= :amount , `types`= :types  
            WHERE `list_id`= :list_id AND user_id = :user_id;",
            params! {
                "list_id" => list_id,
                "description" => description,
                "date" => date,
                "amount" =>amount,
                "types" =>types,
                "user_id" => user_id
            },
        )},
        Err(e) => {
            println!("Failed to get DB connection: {}", e);
            return;
        }
    };
}

//  แก้ไข balance_total หลัก
pub fn edit_balance_total(user_id:i32 ,balance_total_update:i32,types:String){
    
    if types.to_string() == "income"{
        let _ = match con_db() {
            Ok(mut conn) => {
                conn.exec_drop(
                "UPDATE `userdata` SET `balance_total`= `balance_total` + :balance_total_update WHERE `user_id` = :user_id",
                params! {
                    "balance_total_update" => balance_total_update,
                    "user_id" => user_id
                },
            )},
            Err(e) => {
                println!("Failed to get DB connection: {}", e);
                return;
            }
        };
    }else{
        let _ = match con_db() {
            Ok(mut conn) => {
                conn.exec_drop(
                "UPDATE `userdata` SET `balance_total`= `balance_total` - :balance_total_update WHERE `user_id` = :user_id",
                params! {
                    "balance_total_update" => balance_total_update,
                    "user_id" => user_id
                },
            )},
            Err(e) => {
                println!("Failed to get DB connection: {}", e);
                return;
            }
        };
    }
    
    
}