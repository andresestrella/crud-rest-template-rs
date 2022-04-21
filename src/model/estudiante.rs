use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Estudiante {
	pub id: u64,
	pub nombre: String,
	pub telefono: String,
    pub fecha_nacimiento: String
}