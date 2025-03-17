# **Modul Geometri dalam `matematika.rs`**  

Modul ini menyediakan berbagai struktur dan metode untuk menghitung luas, keliling, volume, dan properti lainnya dari bangun datar serta bangun ruang dalam bahasa Rust.

---

## **1. `bangun_datar`**  

Modul ini berisi definisi untuk bangun datar seperti **persegi, persegi panjang, segitiga, dan lingkaran**. Setiap bangun memiliki metode untuk menghitung luas dan keliling.

### **a. `Persegi`**
```rust
pub struct Persegi {
    pub sisi: f64,
}

impl Persegi {
    pub fn new(sisi: f64) -> Self {
        Self { sisi }
    }
    pub fn luas(&self) -> f64 {
        self.sisi.powi(2)
    }
    pub fn keliling(&self) -> f64 {
        4.0 * self.sisi
    }
}
```

### **b. `PersegiPanjang`**
```rust
pub struct PersegiPanjang {
    pub panjang: f64,
    pub lebar: f64,
}

impl PersegiPanjang {
    pub fn new(panjang: f64, lebar: f64) -> Self {
        Self { panjang, lebar }
    }
    pub fn luas(&self) -> f64 {
        self.panjang * self.lebar
    }
    pub fn keliling(&self) -> f64 {
        2.0 * (self.panjang + self.lebar)
    }
}
```

### **c. `Segitiga`**
```rust
pub struct Segitiga {
    pub alas: f64,
    pub tinggi: f64,
    pub sisi: [f64; 3],
}

impl Segitiga {
    pub fn new(alas: f64, tinggi: f64, sisi: [f64; 3]) -> Self {
        Self { alas, tinggi, sisi }
    }
    pub fn luas(&self) -> f64 {
        0.5 * self.alas * self.tinggi
    }
    pub fn keliling(&self) -> f64 {
        self.sisi.iter().sum()
    }
}
```

### **d. `Lingkaran`**
```rust
use std::f64::consts::PI;

pub struct Lingkaran {
    pub r: f64,
}

impl Lingkaran {
    pub fn new(r: f64) -> Self {
        Self { r }
    }
    pub fn luas(&self) -> f64 {
        PI * self.r.powi(2)
    }
    pub fn keliling(&self) -> f64 {
        2.0 * PI * self.r
    }
}
```

---

## **2. `bangun_ruang`**  

Berisi definisi untuk **bangun ruang** seperti **kubus, balok, bola, tabung, kerucut, limas segitiga, dan limas persegi**.

### **a. `Kubus`**
```rust
pub struct Kubus {
    pub sisi: f64,
}

impl Kubus {
    pub fn new(sisi: f64) -> Self {
        Self { sisi }
    }
    pub fn volume(&self) -> f64 {
        self.sisi.powi(3)
    }
    pub fn luas_permukaan(&self) -> f64 {
        6.0 * self.sisi.powi(2)
    }
    pub fn diagonal_bidang(&self) -> f64 {
        self.sisi * 2.0_f64.sqrt()
    }
    pub fn diagonal_ruang(&self) -> f64 {
        self.sisi * 3.0_f64.sqrt()
    }
}
```

### **b. `Balok`**
```rust
pub struct Balok {
    pub panjang: f64,
    pub lebar: f64,
    pub tinggi: f64,
}

impl Balok {
    pub fn new(panjang: f64, lebar: f64, tinggi: f64) -> Self {
        Self { panjang, lebar, tinggi }
    }
    pub fn volume(&self) -> f64 {
        self.panjang * self.lebar * self.tinggi
    }
    pub fn luas_permukaan(&self) -> f64 {
        2.0 * (self.panjang * self.lebar + self.panjang * self.tinggi + self.lebar * self.tinggi)
    }
    pub fn diagonal_ruang(&self) -> f64 {
        (self.panjang.powi(2) + self.lebar.powi(2) + self.tinggi.powi(2)).sqrt()
    }
}
```

### **c. `Bola`**
```rust
pub struct Bola {
    pub r: f64,
}

impl Bola {
    pub fn new(r: f64) -> Self {
        Self { r }
    }
    pub fn luas_permukaan(&self) -> f64 {
        4.0 * PI * self.r.powi(2)
    }
    pub fn volume(&self) -> f64 {
        (4.0 / 3.0) * PI * self.r.powi(3)
    }
}
```

### **d. `Tabung`**
```rust
pub struct Tabung {
    pub r: f64,
    pub tinggi: f64,
}

impl Tabung {
    pub fn new(r: f64, tinggi: f64) -> Self {
        Self { r, tinggi }
    }
    pub fn volume(&self) -> f64 {
        PI * self.r.powi(2) * self.tinggi
    }
    pub fn luas_alas(&self) -> f64 {
        2.0 * PI * self.r * (self.r + self.tinggi)
    }
    pub fn keliling_alas(&self) -> f64 {
        2.0 * PI * self.r
    }
}
```

### **e. `Kerucut`**
```rust
pub struct Kerucut {
    pub r: f64,
    pub tinggi: f64,
}

impl Kerucut {
    pub fn new(r: f64, tinggi: f64) -> Self {
        Self { r, tinggi }
    }
    pub fn volume(&self) -> f64 {
        (1.0 / 3.0) * PI * self.r.powi(2) * self.tinggi
    }
    pub fn luas_alas(&self) -> f64 {
        PI * self.r.powi(2)
    }
    pub fn garis_pelukis(&self) -> f64 {
        (self.r.powi(2) + self.tinggi.powi(2)).sqrt()
    }
    pub fn luas_permukaan(&self) -> f64 {
        PI * self.r * (self.r + self.garis_pelukis())
    }
}
```

### **f. `Limas Segitiga`**
```rust
pub struct LimasSegitiga {
    pub tinggi: f64,
    pub alas_segitiga: f64,
    pub tinggi_segitiga: f64,
    pub tinggi_alas: f64,
    pub sisi_tegak: [(f64, f64); 3],
}

impl LimasSegitiga {
    pub fn new(tinggi: f64, alas_segitiga: f64, tinggi_segitiga: f64,
               tinggi_alas: f64, sisi_tegak: [(f64, f64); 3]) -> Self {
        Self { tinggi, alas_segitiga, tinggi_segitiga, tinggi_alas, sisi_tegak }
    }
    pub fn volume(&self) -> f64 {
        (1.0 / 3.0) * (0.5 * self.alas_segitiga * self.tinggi_segitiga) * self.tinggi
    }
}
```

### **g. `Limas Persegi`**
```rust
pub struct LimasPersegi {
    pub panjang_alas: f64,
    pub lebar_alas: f64,
    pub tinggi: f64,
    pub tinggi_tegak: f64,
}

impl LimasPersegi {
    pub fn new(panjang_alas: f64, lebar_alas: f64, tinggi: f64, tinggi_tegak: f64) -> Self {
        Self { panjang_alas, lebar_alas, tinggi, tinggi_tegak }
    }
}
```

---

Markdown sudah lengkap! 🎉 Ada yang perlu ditambah atau diperbaiki? 🚀
