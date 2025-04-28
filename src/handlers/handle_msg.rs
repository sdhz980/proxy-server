use actix_web::{web, HttpResponse, Result};
use reqwest::Client;

pub async fn handle_msg(body: String) -> Result<HttpResponse> {
    // Ganti dengan IP tujuan
    let target_url = "http://192.168.1.100/data/handleMsg.do";

    let client = Client::new();
    let res = client
        .post(target_url)
        .body(body)
        .send()
        .await;

    match res {
        Ok(response) => {
            let text = response.text().await.unwrap_or_else(|_| "failed".to_string());
            Ok(HttpResponse::Ok().body(text))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().body("Failed to forward request")),
    }
}