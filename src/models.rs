use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::schema::*;

// ==== Client ====

#[derive(Queryable, Debug)]
pub struct Client {
    pub id: i32,
    pub id_client: String,
    pub owner: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = client)]
pub struct NewClient<'a> {
    pub id_client: &'a str,
    pub owner: &'a str,
}

// ==== Http ====

#[derive(Queryable, Debug)]
pub struct Http {
    pub id: i32,
    pub data: String,
    pub created_at: NaiveDateTime,
    pub client_id: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = http)]
pub struct NewHttp<'a> {
    pub data: &'a str,
    pub client_id: Option<&'a str>,
}

// ==== IdGame ====

#[derive(Queryable, Debug)]
pub struct IdGame {
    pub id: i32,
    pub data: String,
    pub created_at: NaiveDateTime,
    pub client_id: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = id_game)]
pub struct NewIdGame<'a> {
    pub data: &'a str,
    pub client_id: Option<&'a str>,
}

// ==== LocalData ====

#[derive(Queryable, Debug)]
pub struct LocalData {
    pub id: i32,
    pub data: String,
    pub created_at: NaiveDateTime,
    pub client_id: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = local_data)]
pub struct NewLocalData<'a> {
    pub data: &'a str,
    pub client_id: Option<&'a str>,
}

// ==== Log ====

#[derive(Queryable, Debug)]
pub struct Log {
    pub id: i32,
    pub data: String,
    pub created_at: NaiveDateTime,
    pub client_id: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = log)]
pub struct NewLog<'a> {
    pub data: &'a str,
    pub client_id: Option<&'a str>,
}

// ==== Mac ====

#[derive(Queryable, Debug)]
pub struct Mac {
    pub id: i32,
    pub data: String,
    pub created_at: NaiveDateTime,
    pub client_id: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = mac)]
pub struct NewMac<'a> {
    pub data: &'a str,
    pub client_id: Option<&'a str>,
}