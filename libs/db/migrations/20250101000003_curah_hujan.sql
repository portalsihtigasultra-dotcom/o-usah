CREATE TABLE curah_hujan (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pos_id          UUID NOT NULL REFERENCES pos_pengamatan(id) ON DELETE RESTRICT,
    tanggal         DATE NOT NULL,
    nilai_mm        REAL NOT NULL,
    jam_pengukuran  TIME NOT NULL,
    petugas_id      UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    status          status_data_enum NOT NULL DEFAULT 'mentah',
    keterangan      TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ch_pos_id ON curah_hujan(pos_id);
CREATE INDEX idx_ch_tanggal ON curah_hujan(tanggal);
CREATE INDEX idx_ch_status ON curah_hujan(status);
CREATE INDEX idx_ch_petugas ON curah_hujan(petugas_id);
CREATE INDEX idx_ch_pos_tanggal ON curah_hujan(pos_id, tanggal);