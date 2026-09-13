use crate::models::tanaman::Tanaman;
use std::sync::Mutex;

pub struct AppState {
    pub koleksi: Mutex<Vec<Tanaman>>,
    pub next_id: Mutex<u32>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            koleksi: Mutex::new(vec![
                Tanaman {
                    id: 1,
                    nama: String::from("Monstera"),
                    jenis: String::from("Hias"),
                    tinggi_cm: 60,
                },
                Tanaman {
                    id: 2,
                    nama: String::from("Lidah Buaya"),
                    jenis: String::from("Obat"),
                    tinggi_cm: 30,
                },
                Tanaman {
                    id: 3,
                    nama: String::from("Kaktus"),
                    jenis: String::from("Sukulen"),
                    tinggi_cm: 15,
                },
            ]),
            next_id: Mutex::new(4),
        }
    }
}
