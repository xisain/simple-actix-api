use crate::errors::AppError;
use crate::models::tanaman::{TanamanBaru, TanamanUpdate};
use crate::state::AppState;
use actix_web::{HttpResponse, delete, get, post, put, web};

// GET /tanaman
// Ambil semua tanaman. Opsional filter: ?jenis=Hias
#[derive(serde::Deserialize)]
pub struct QueryFilter {
    pub jenis: Option<String>,
}

#[get("/tanaman")]
pub async fn semua_tanaman(
    data: web::Data<AppState>,
    query: web::Query<QueryFilter>,
) -> HttpResponse {
    let koleksi = data.koleksi.lock().unwrap();

    let hasil: Vec<_> = koleksi
        .iter()
        .filter(|t| {
            query
                .jenis
                .as_ref()
                .map(|j| t.jenis.to_lowercase() == j.to_lowercase())
                .unwrap_or(true)
        })
        .collect();

    HttpResponse::Ok().json(hasil)
}

// GET /tanaman/{id}
// Ambil satu tanaman berdasarkan id
#[get("/tanaman/{id}")]
pub async fn satu_tanaman(
    data: web::Data<AppState>,
    id: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    let koleksi = data.koleksi.lock().unwrap();

    let tanaman = koleksi.iter().find(|t| t.id == *id);

    match tanaman {
        Some(t) => Ok(HttpResponse::Ok().json(t)),
        None => Err(AppError::TidakDitemukan(format!(
            "Tanaman dengan id {} tidak ditemukan",
            id
        ))),
    }
}

// POST /tanaman
// Tambah tanaman baru
#[post("/tanaman")]
pub async fn tambah_tanaman(
    data: web::Data<AppState>,
    body: web::Json<TanamanBaru>,
) -> Result<HttpResponse, AppError> {
    let baru = body.into_inner();

    if baru.nama.trim().is_empty() {
        return Err(AppError::InputTidakValid(
            "Nama tanaman tidak boleh kosong".to_string(),
        ));
    }

    let mut koleksi = data.koleksi.lock().unwrap();
    let mut next_id = data.next_id.lock().unwrap();

    let tanaman = crate::models::tanaman::Tanaman {
        id: *next_id,
        nama: baru.nama,
        jenis: baru.jenis,
        tinggi_cm: baru.tinggi_cm,
    };

    *next_id += 1;
    koleksi.push(tanaman.clone());

    Ok(HttpResponse::Created().json(tanaman))
}

// PUT /tanaman/{id}
// Update tanaman — hanya field yang dikirim yang diubah
#[put("/tanaman/{id}")]
pub async fn update_tanaman(
    data: web::Data<AppState>,
    id: web::Path<u32>,
    body: web::Json<TanamanUpdate>,
) -> Result<HttpResponse, AppError> {
    let perubahan = body.into_inner();
    let mut koleksi = data.koleksi.lock().unwrap();

    let tanaman = koleksi.iter_mut().find(|t| t.id == *id);

    match tanaman {
        Some(t) => {
            if let Some(nama) = perubahan.nama {
                t.nama = nama;
            }
            if let Some(jenis) = perubahan.jenis {
                t.jenis = jenis;
            }
            if let Some(tinggi) = perubahan.tinggi_cm {
                t.tinggi_cm = tinggi;
            }
            Ok(HttpResponse::Ok().json(t.clone()))
        }
        None => Err(AppError::TidakDitemukan(format!(
            "Tanaman dengan id {} tidak ditemukan",
            id
        ))),
    }
}

// DELETE /tanaman/{id}
// Hapus tanaman berdasarkan id
#[delete("/tanaman/{id}")]
pub async fn hapus_tanaman(
    data: web::Data<AppState>,
    id: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    let mut koleksi = data.koleksi.lock().unwrap();
    let sebelum = koleksi.len();

    koleksi.retain(|t| t.id != *id);

    if koleksi.len() < sebelum {
        Ok(HttpResponse::Ok().json(format!("Tanaman id {} berhasil dihapus", id)))
    } else {
        Err(AppError::TidakDitemukan(format!(
            "Tanaman dengan id {} tidak ditemukan",
            id
        )))
    }
}
