-- 002_pos_pengamatan.sql
CREATE TABLE pos_pengamatan (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    kode_pos    VARCHAR(16) UNIQUE NOT NULL,
    nama_pos    VARCHAR(128) NOT NULL,
    tipe_pos    tipe_pos_enum NOT NULL,
    das         VARCHAR(64),
    lat         DOUBLE PRECISION,
    long        DOUBLE PRECISION,
    elevasi     DOUBLE PRECISION,
    kabupaten   VARCHAR(64),
    kecamatan   VARCHAR(64),
    status_aktif BOOLEAN NOT NULL DEFAULT TRUE,
    keterangan  TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pos_kode ON pos_pengamatan(kode_pos);
CREATE INDEX idx_pos_tipe ON pos_pengamatan(tipe_pos);
CREATE INDEX idx_pos_kabupaten ON pos_pengamatan(kabupaten);