mod config;
mod model;
mod service;
mod repository;

use dotenv::dotenv;
use actix_web::{HttpServer,App,web, HttpResponse};
use sqlx::{MySql, Pool};
use crate::repository::*;

pub type Db = Pool<MySql>;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    //let con_string = format!("postgres://{}:{}@{}/{}", config.mariadb.root_user, config.mariadb.root_password, config.mariadb.host, config.mariadb.database);
    //let pool = MySqlPoolOptions::new()
        //.max_connections(5)
        //.connect("mysql://root:example@localhost/api-db").await?; //hardcoded, constring or env var route
 
    println!("Starting server at http://{}:{}/", config.server.host, config.server.port);

    let estudiante_repository = EstudianteRepository::from_env().await.expect("No se pudo conectar a la base de datos");
    let estudiante_repository = web::Data::new(estudiante_repository);

    HttpServer::new( move|| {
        App::new()
        .app_data(estudiante_repository.clone())
            .route("/health{_:/?}", web::get().to(health_handler))
            .configure(service::estudiante_service)
            })
        .bind(format!("{}:{}", config.server.host, config.server.port))
        .unwrap_or_else(|err| {
            panic!("No pudo iniciar servidor en puerto {}: {:?}", config.server.port, err)
        })
        .run()
        .await
}

async fn health_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}