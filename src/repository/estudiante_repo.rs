use async_trait::async_trait;
use super::RepositoryError;
use crate::{repository, model::Estudiante};
use crate::repository::crud::Crud;

pub struct EstudianteRepository{
    pool: sqlx::MySqlPool,
}

impl EstudianteRepository {
	const TABLE: &'static str = "estudiante";

    pub async fn from_env() -> sqlx::Result<Self>{
        let connection_str = std::env::var("DATABASE_URL").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        let pool = sqlx::MySqlPool::connect(&connection_str ).await?;
        Ok(Self{pool})
    }
}

// Estudiante Model Access Controller
#[async_trait]
impl Crud<Estudiante> for EstudianteRepository {
 
	async fn create(&self, data: &Estudiante) -> Result<Estudiante, repository::RepositoryError> {
		let result = sqlx::query_as::<_, Estudiante>(
            r#"
        INSERT INTO estudiante (id, nombre, telefono, fecha_nacimiento)
        VALUES (?, ?, ?, ?)"#,
        )
        .bind(&data.id)
        .bind(&data.nombre)
        .bind(&data.telefono)
        .bind(&data.fecha_nacimiento)
        .fetch_one(&self.pool)
        .await;

		result.map_err(|_e|{
            RepositoryError::AlreadyExists
        })
	}

	async fn get(&self, id: u64) -> Result<Estudiante, repository::RepositoryError> {
		let result = sqlx::query_as::<_, Estudiante>(
            "SELECT id, nombre, telefono, fecha_nacimiento 
            FROM estudiante
            WHERE id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await;

        handle_fetch_one_result(result, Self::TABLE, id)
	}

	async fn update(&self, data: &Estudiante) -> Result<Estudiante, repository::RepositoryError> {
		let result = sqlx::query_as::<_, Estudiante>(
            r#"
            UPDATE estudiante
            SET nombre = ?, telefono = ?, fecha_nacimiento = ?
            WHERE id = ?
            "#,
        )
        .bind(&data.nombre)
        .bind(&data.telefono)
        .bind(&data.fecha_nacimiento)
        .bind(&data.id)
        .fetch_one(&self.pool)
        .await;

		handle_fetch_one_result(result, Self::TABLE, data.id)
	}

	async fn get_all(&self) -> Result<Vec<Estudiante>, repository::RepositoryError> {
		let result = sqlx::query_as::<_, Estudiante>(
            r#"
            SELECT * FROM estudiante
        "#,
        ).fetch_all(&self.pool).await;

		// execute the query
		result.map_err(|_e|{
            RepositoryError::EmptyTable
        })
	}

	async fn delete(&self, id: u64) -> Result<Estudiante, repository::RepositoryError> {
		let result = sqlx::query_as::<_, Estudiante>(
            r#"
            DELETE FROM estudiante
            WHERE id = ?
        "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await;

		handle_fetch_one_result(result, Self::TABLE, id)
	}
}
// endregion: EstudianteRepository

// region:    Utils
fn handle_fetch_one_result(result: Result<Estudiante, sqlx::Error>, typ: &'static str, id: u64,) -> Result<Estudiante, repository::RepositoryError> {
	result.map_err(|sqlx_error| match sqlx_error {
		sqlx::Error::RowNotFound => repository::RepositoryError::EntityNotFound(typ, id.to_string()),
		other => repository::RepositoryError::SqlxError(other),
	})
}
// endregion: Utils