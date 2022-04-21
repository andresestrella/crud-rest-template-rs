mod estudiante;

//use crate::{repository::{Crud}, model::Estudiante};
use actix_web::web::{self, ServiceConfig};

pub fn estudiante_service(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/apiv1").configure(estudiante::service));
}