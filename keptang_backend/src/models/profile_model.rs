use serde::{Serialize, Deserialize};
use crate::config::db::con_db;
use mysql::*;
use mysql::prelude::*;


// profile_input
#[derive(Serialize, Deserialize)]
pub struct ProfileRequest {
    pub id: i32
}

//profile_output
#[derive(Serialize, Deserialize)]
pub struct ProfileResponse {
    name: String,
    balance_total: i32,
    id: i32
}


// profile-return-database
pub fn get_user_profile(user_id:i32) -> Vec <ProfileResponse>{
    let db = con_db()
    .map(|mut conn| {
        conn.query_map(
            "SELECT `user_id`, `user_name`, `balance_total` FROM `userdata` WHERE `user_id`= ".to_owned() + user_id.to_string().as_str(),
            |(id,name,balance_total)| {
                ProfileResponse
                {
                    id,
                    name,
                    balance_total
                    
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
    let mut data:Vec <ProfileResponse> = Default::default();
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
