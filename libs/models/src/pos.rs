use chrono::{DateTime, Utc};
use o_usah_core::DbId;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "tipe_pos_enum", rename_all = "snake_case")]
pub enum TipePos {
    CurahHujan,
    Debit,
    Klimatologi,
    KualitasAir,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PosPengamatan {
    pub id: DbId,
    pub kode_pos: String,
    pub nama_pos: String,
    pub tipe_pos: TipePos,
    pub das: String,
    pub lat: f32,
    pub long: f32,
    pub elevasi: Option<f32>,
    pub kabupaten: String,
    pub kecamatan: Option<String>,
    pub status_aktif: Option<bool>,
    pub keterangan: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePosDto {
    pub kode_pos: String,
    pub nama_pos: String,
    pub tipe_pos: TipePos,
    pub das: String,
    pub lat: f32,
    pub long: f32,
    pub elevasi: Option<f32>,
    pub kabupaten: String,
    pub kecamatan: Option<String>,
    pub keterangan: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePosDto {
    pub nama_pos: Option<String>,
    pub das: Option<String>,
    pub lat: Option<f32>,
    pub long: Option<f32>,
    pub elevasi: Option<f32>,
    pub kabupaten: Option<String>,
    pub kecamatan: Option<String>,
    pub status_aktif: Option<bool>,
    pub keterangan: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PosResponse {
    pub id: DbId,
    pub kode_pos: String,
    pub nama_pos: String,
    pub tipe_pos: TipePos,
    pub das: String,
    pub lat: f32,
    pub long: f32,
    pub elevasi: Option<f32>,
    pub kabupaten: String,
    pub kecamatan: Option<String>,
    pub status_aktif: Option<bool>,
    pub keterangan: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PosPengamatan> for PosResponse {
    fn from(p: PosPengamatan) -> Self {
        Self {
            id: p.id,
            kode_pos: p.kode_pos,
            nama_pos: p.nama_pos,
            tipe_pos: p.tipe_pos,
            das: p.das,
            lat: p.lat,
            long: p.long,
            elevasi: p.elevasi,
            kabupaten: p.kabupaten,
            kecamatan: p.kecamatan,
            status_aktif: p.status_aktif,
            keterangan: p.keterangan,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}