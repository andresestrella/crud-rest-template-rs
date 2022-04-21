mod estudiante_repo;
mod crud;

pub use estudiante_repo::{EstudianteRepository};
pub use crud::Crud;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
	#[error("Entity Not Found - {0}[{1}] ")]
	EntityNotFound(&'static str, String),

	#[error(transparent)]
	SqlxError(#[from] sqlx::Error),

	#[error(transparent)]
	IOError(#[from] std::io::Error),

    #[error("This entity already exists")]
    AlreadyExists,

    #[error("This table doesn't contain anything in it")]
    EmptyTable,
}