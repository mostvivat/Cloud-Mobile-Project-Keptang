use serde::{Serialize, Deserialize};
use crate::config::db::con_db;
use mysql::*;
use mysql::prelude::*;
use chrono::{Local, DateTime};

// ตารางเก็บ บัญชีผู้ใช้  
#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub user_name: String,
    pub balance_total: i32,
}
#[derive(Serialize, Deserialize,Clone)]
// เป็น object ที่เราสร้างขึ้นมา  ตารางเก็บ รายรับ-รายจ่าย  ใช้หน้า money
pub struct MoneyList {
    pub list_id: i32,
    pub description: String,
    pub date: String,
    pub amount: i32,
    pub types: String,
}

// money_input
#[derive(Serialize, Deserialize)]
pub struct MoneyRequest {
    pub id: i32
}

//money_output
#[derive(Serialize, Deserialize)]
pub struct MoneyResponse  {
    pub name: String,
    pub balance_total: i32,
    pub balance_today: i32, 
    pub items: Vec<MoneyList>
}
// ไว้หา moneylist ที่จะเปลี่ยนค่า รวม
#[derive(Serialize, Deserialize)]
pub  struct MoneyResponseByID {
    pub user_id: i32,
    pub amount: i32,
    pub types: String
}

// money-return-database
pub fn get_user_money(user_id:i32) -> Vec <MoneyResponse>{
    //get data from userdata table
    let db1 = con_db()
    .map(|mut conn| {
        conn.query_map(
            "SELECT  `user_name`, `balance_total` FROM `userdata` WHERE `user_id`= ".to_owned() + user_id.to_string().as_str(),
            |(user_name,balance_total)|{
            UserData
                {
                    user_name,
                    balance_total
                }
            },
        )
    })
   .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
    //get data from MoneyList table 
    let db2 = con_db()
    .map(|mut conn| {
        conn.query_map(
            "SELECT moneylist.list_id,moneylist.description,moneylist.date,moneylist.amount,moneylist.types 
            FROM moneylist INNER JOIN userdata ON moneylist.user_id = userdata.user_id 
            WHERE moneylist.user_id =  ".to_owned() + user_id.to_string().as_str(),
            |(list_id,description,date,amount,types)|{
                MoneyList
                {
                    list_id,
                    description,
                    date,
                    amount,
                    types
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
    //combine data1 and data2
    let mut data3: Vec<MoneyResponse> = Default::default();
    let mut balance_today_calculator = 0;
    let time_now: DateTime<Local> = Local::now();
    let formatted = time_now.format("%Y-%m-%d").to_string();
    match (db1, db2) {
        (Ok(db1), Ok(db2)) => {

            let db_clone = db2.clone();
            for j in db_clone{
                if j.types == "income" && j.date == formatted{
                    balance_today_calculator += j.amount;
                }else if j.types == "expense" && j.date == formatted{
                    balance_today_calculator -= j.amount;
                }
            }

            for i in db1 {
                data3.push(MoneyResponse {
                    name: i.user_name.clone(),
                    balance_total: i.balance_total,
                    balance_today:balance_today_calculator,
                    items: db2.clone(),
                });
            }
        }
        (Err(e), _) => println!("Error: {}", e),
        (_, Err(e)) => println!("Error: {}", e),
    }

    return data3;
}  

pub fn get_moneylist_byid(user_id:i32,list_id:i32) -> Vec<MoneyResponseByID>{

    let sql_db = format!("SELECT moneylist.user_id,moneylist.amount,moneylist.types 
    FROM moneylist INNER JOIN userdata ON moneylist.user_id = userdata.user_id 
    WHERE moneylist.user_id =  {} AND moneylist.list_id = {}", user_id, list_id);

    let db = con_db()
    .map(|mut conn| {
        conn.query_map(
            sql_db,
            |(u_id,amount,types)|{
                MoneyResponseByID
                {
                    user_id: u_id,
                    amount: amount,
                    types: types
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
    let mut data:Vec <MoneyResponseByID> = Default::default();
    match db {
        Ok(money) => {
            for i in money{
                data.push(i);
            }
        }
        Err(e) => println!("Error: {}", e)
    }
    return data;
}