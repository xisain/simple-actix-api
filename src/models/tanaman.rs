use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tanaman {
    pub id: u32,
    pub nama: String,
    pub jenis: String,
    pub tinggi_cm: u32,
}

// Payload untuk POST — tanpa id (id di-generate server)
#[derive(Deserialize)]
pub struct TanamanBaru {
    pub nama: String,
    pub jenis: String,
    pub tinggi_cm: u32,
}

// Payload untuk PUT — semua field opsional
#[derive(Deserialize)]
pub struct TanamanUpdate {
    pub nama: Option<String>,
    pub jenis: Option<String>,
    pub tinggi_cm: Option<u32>,
}
