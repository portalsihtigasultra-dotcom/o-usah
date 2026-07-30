use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use o_usah_core::DbId;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "status_data_enum", rename_all = "snake_case")]
pub enum StatusData {
    Mentah,
    Tervalidasi,
    Ditolak,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CurahHujan {
    pub id: DbId,
    pub pos_id: DbId,
    pub tanggal: NaiveDate,
    pub nilai_mm: f32,
    pub jam_pengukuran: NaiveTime,
    pub petugas_id: DbId,
    pub status: StatusData,
    pub keterangan: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCurahHujanDto {
    pub pos_id: DbId,
    pub tanggal: NaiveDate,
    pub nilai_mm: f32,
    pub jam_pengukuran: NaiveTime,
    pub keterangan: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCurahHujanDto {
    pub nilai_mm: Option<f32>,
    pub jam_pengukuran: Option<NaiveTime>,
    pub keterangan: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateStatusDto {
    pub status: StatusData,
    pub keterangan: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CurahHujanResponse {
    pub id: DbId,
    pub pos_id: DbId,
    pub tanggal: NaiveDate,
    pub nilai_mm: f32,
    pub jam_pengukuran: NaiveTime,
    pub petugas_id: DbId,
    pub status: StatusData,
    pub keterangan: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<CurahHujan> for CurahHujanResponse {
    fn from(ch: CurahHujan) -> Self {
        Self {
            id: ch.id,
            pos_id: ch.pos_id,
            tanggal: ch.tanggal,
            nilai_mm: ch.nilai_mm,
            jam_pengukuran: ch.jam_pengukuran,
            petugas_id: ch.petugas_id,
            status: ch.status,
            keterangan: ch.keterangan,
            created_at: ch.created_at,
            updated_at: ch.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CurahHujanFilter {
    pub pos_id: Option<DbId>,
    pub tanggal_mulai: Option<NaiveDate>,
    pub tanggal_akhir: Option<NaiveDate>,
    pub status: Option<StatusData>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}