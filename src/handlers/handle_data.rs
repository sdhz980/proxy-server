use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::schema::{http, id_game, mac, local_data};

#[derive(Deserialize)]
pub struct QueryParams {
    #[serde(rename = "clientId")]
    client_id: String,
}

#[derive(Deserialize)]
pub struct InputData {
    data: String,
}

pub async fn handle_data(
    path: web::Path<u32>,
    query: web::Query<QueryParams>,
    json: web::Json<InputData>,
) -> Result<HttpResponse> {
    let conn = &mut establish_connection();
    let id = path.into_inner();
    let client_id = &query.client_id;
    let data = &json.data;

    match id {
        1 => {
            // Simpan ke table `http`
            diesel::insert_into(http::table)
                .values((
                    http::data.eq(data),
                    http::client_id.eq(client_id),
                ))
                .execute(conn)
                .map_err(|_| HttpResponse::InternalServerError())?;

            Ok(HttpResponse::Ok().body("http saved"))
        }
        2 => {
            // Simpan ke table `id_game`
            diesel::insert_into(id_game::table)
                .values((
                    id_game::data.eq(data),
                    id_game::client_id.eq(client_id),
                ))
                .execute(conn)
                .map_err(|_| HttpResponse::InternalServerError())?;

            Ok(HttpResponse::Ok().body("id_game saved"))
        }
        3 => {
            // Simpan ke table `mac`
            diesel::insert_into(mac::table)
                .values((
                    mac::data.eq(data),
                    mac::client_id.eq(client_id),
                ))
                .execute(conn)
                .map_err(|_| HttpResponse::InternalServerError())?;

            Ok(HttpResponse::Ok().body("mac saved"))
        }
        4 => {
            // Simpan ke table `local_data`
            diesel::insert_into(local_data::table)
                .values((
                    local_data::data.eq(data),
                    local_data::client_id.eq(client_id),
                ))
                .execute(conn)
                .map_err(|_| HttpResponse::InternalServerError())?;

            Ok(HttpResponse::Ok().body("local_data saved"))
        }
        _ => Ok(HttpResponse::BadRequest().body("Invalid ID")),
    }
}