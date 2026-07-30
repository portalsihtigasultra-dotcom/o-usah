use chrono::{DateTime, Utc};
use o_usah_core::DbId;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "user_role_enum", rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    Staf,
    PetugasLapangan,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: DbId,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
    pub nama_lengkap: String,
    pub email: Option<String>,
    pub status_aktif: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub password: String,
    pub role: UserRole,
    pub nama_lengkap: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: DbId,
    pub username: String,
    pub role: UserRole,
    pub nama_lengkap: String,
    pub email: Option<String>,
    pub status_aktif: bool,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            username: u.username,
            role: u.role,
            nama_lengkap: u.nama_lengkap,
            email: u.email,
            status_aktif: u.status_aktif,
            created_at: u.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}