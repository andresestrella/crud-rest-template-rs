use actix_web::{
    error::PathError,
    web::{self, PathConfig},
    HttpRequest, HttpResponse,
};

use web::ServiceConfig;
use crate::{repository::{EstudianteRepository, Crud}, model::Estudiante};

const PATH: &str = "/estudiante";

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope(PATH)
            .app_data(PathConfig::default().error_handler(path_config_handler))
            // GET
            .route("/{id}", web::get().to(get))
            //GET ALL
            .route("/", web::get().to(get_all))
            // POST
            .route("/", web::post().to(post))
            // PUT
            .route("/", web::put().to(put))
            // DELETE
            .route("/{id}", web::delete().to(delete))
    );
}

async fn get (id: web::Path<u64>, repo: web::Data<EstudianteRepository>) -> HttpResponse { //req: HttpRequest
    match repo.get(&id).await {
        Ok(estudiante) => HttpResponse::Ok().json(estudiante),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

async fn get_all (repo: web::Data<EstudianteRepository>) -> HttpResponse {
    match repo.get_all().await {
        Ok(estudiantes) => HttpResponse::Ok().json(estudiantes),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

async fn post (estudiante: web::Json<Estudiante>, repo: web::Data<EstudianteRepository>) -> HttpResponse {
    match repo.create(&estudiante).await {
        Ok(estudiante) => HttpResponse::Created().json(estudiante),
        Err(e) => HttpResponse::InternalServerError().body(format!("Something went wrong: {}", e)),
    }
}


async fn put (estudiante: web::Json<Estudiante>, repo: web::Data<EstudianteRepository>) -> HttpResponse {
    match repo.update(&estudiante).await {
        Ok(estudiante) => HttpResponse::Ok().json(estudiante),
        Err(e) => HttpResponse::NotFound().body(format!("Something went wrong: {}", e)),
    }
}


async fn delete (id: web::Path<u64>, repo: web::Data<EstudianteRepository>) -> HttpResponse {
    match repo.delete(*id).await {
        Ok(estudiante) => HttpResponse::Ok().body(format!("Estudiante of ID:{} deleted", estudiante)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Something went wrong: {}", e)),
    }
}

fn path_config_handler(err: PathError, _req: &HttpRequest) -> actix_web::Error {
    actix_web::error::ErrorBadRequest(err)
}