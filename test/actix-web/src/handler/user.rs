use actix_web::web::Data;
use actix_web::{delete, get, post, web, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};
use web::{Json, Path};

use crate::handler::result::{none, some};

#[derive(Serialize)]
struct GetUserResult {
    id: i64,
    name: Option<String>,
}

#[derive(Deserialize)]
pub struct SaveUserQuery {
    id: Option<i64>,
    name: Option<String>,
}

#[get("user")]
pub async fn all(data: Data<PgPool>) -> impl Responder {
    some(
        query_as!(
            GetUserResult,
            r#"select id, name from "user" where deleted_at is null"#
        )
        .fetch_all(data.get_ref())
        .await,
    )
}

#[get("user/{id}")]
pub async fn get(data: Data<PgPool>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    some(
        query_as!(
            GetUserResult,
            r#"select id, name from "user" where deleted_at is null and id = $1"#,
            id
        )
        .fetch_one(data.get_ref())
        .await,
    )
}

#[post("user")]
pub async fn post(data: Data<PgPool>, Json(v): Json<SaveUserQuery>) -> impl Responder {
    none(
        match v.id {
            None => query!(
                r#"insert into "user" (name, created_at) values ($1, now())"#,
                v.name
            ).execute(data.get_ref()).await,
            Some(id) => query!(
                r#"update "user" set name = $1, updated_at = now() where deleted_at is null and id = $2"#,
                v.name,
                id
            ).execute(data.get_ref()).await,
        }
    )
}

#[delete("user/{id}")]
pub async fn delete(data: Data<PgPool>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    none(
        query!(r#"update "user" set deleted_at = now() where id = $1"#, id)
            .execute(data.get_ref())
            .await,
    )
}
