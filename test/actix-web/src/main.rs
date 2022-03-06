use std::env;
use std::error::Error;
use std::result::Result as StdResult;

use actix_web::web::{route, Data};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::info;
use sqlx::PgPool;

mod handler;

pub type Result<T = (), E = Box<dyn Error>> = StdResult<T, E>;

#[actix_web::main]
async fn main() -> Result {
    dotenv().ok();
    log4rs::init_file("config/log4rs.yaml", Default::default())?;
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    info!(target: "main", "程序启动");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .default_service(route().to(handler::default_route))
            .configure(handler::route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
