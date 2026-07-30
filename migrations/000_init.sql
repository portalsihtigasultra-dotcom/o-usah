-- 000_init.sql
-- Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enum: tipe pos pengamatan
CREATE TYPE tipe_pos_enum AS ENUM (
    'curah_hujan',
    'debit_tma',
    'klimatologi',
    'kualitas_air'
);

-- Enum: user role
CREATE TYPE user_role_enum AS ENUM (
    'admin',
    'staf',
    'petugas_lapangan'
);

-- Enum: status data
CREATE TYPE status_data_enum AS ENUM (
    'draft',
    'menunggu',
    'tervalidasi',
    'ditolak'
);