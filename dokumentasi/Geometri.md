# Modul Geometri dalam `matematika.rs`

Modul ini menyediakan berbagai struktur dan metode untuk menghitung luas, keliling, volume, dan properti lainnya dari bangun datar serta bangun ruang dalam bahasa Rust.

## 1. `bangun_datar`

Modul ini berisi definisi untuk bangun datar seperti persegi, persegi panjang, segitiga, dan lingkaran. Setiap bangun memiliki metode untuk menghitung luas dan keliling.

### a. `Persegi`
```rust
struct Persegi {
    sisi: f64,
}

impl Persegi {
    fn new(sisi: f64) -> Self {
        Self { sisi }
    }
    fn luas(&self) -> f64 {
        self.sisi.powi(2)
    }
    fn keliling(&self) -> f64 {
        4.0 * self.sisi
    }
}
```

### b. `PersegiPanjang`
```rust
struct PersegiPanjang {
    panjang: f64,
    lebar: f64,
}

impl PersegiPanjang {
    fn new(panjang: f64, lebar: f64) -> Self {
        Self { panjang, lebar }
    }
    fn luas(&self) -> f64 {
        self.panjang * self.lebar
    }
    fn keliling(&self) -> f64 {
        2.0 * (self.panjang + self.lebar)
    }
}
```

### c. `Segitiga`
```rust
struct Segitiga {
    alas: f64,
    tinggi: f64,
    sisi: [f64; 3],
}

impl Segitiga {
    fn new(alas: f64, tinggi: f64, sisi: [f64; 3]) -> Self {
        Self { alas, tinggi, sisi }
    }
    fn luas(&self) -> f64 {
        0.5 * self.alas * self.tinggi
    }
    fn keliling(&self) -> f64 {
        self.sisi.iter().sum()
    }
}
```

### d. `Lingkaran`
```rust
use std::f64::consts::PI;

struct Lingkaran {
    r: f64,
}

impl Lingkaran {
    fn new(r: f64) -> Self {
        Self { r }
    }
    fn luas(&self) -> f64 {
        PI * self.r.powi(2)
    }
    fn keliling(&self) -> f64 {
        2.0 * PI * self.r
    }
}
```

---

## 2. `bangun_ruang`

Berisi definisi untuk bangun ruang seperti kubus, balok, dan bola.

### a. `Kubus`
```rust
struct Kubus {
    sisi: f64,
}

impl Kubus {
    fn new(sisi: f64) -> Self {
        Self { sisi }
    }
    fn volume(&self) -> f64 {
        self.sisi.powi(3)
    }
    fn luas_permukaan(&self) -> f64 {
        6.0 * self.sisi.powi(2)
    }
    fn diagonal_bidang(&self) -> f64 {
        self.sisi * 2.0_f64.sqrt()
    }
    fn diagonal_ruang(&self) -> f64 {
        self.sisi * 3.0_f64.sqrt()
    }
}
```

### b. `Balok`
```rust
struct Balok {
    panjang: f64,
    lebar: f64,
    tinggi: f64,
}

impl Balok {
    fn new(panjang: f64, lebar: f64, tinggi: f64) -> Self {
        Self { panjang, lebar, tinggi }
    }
    fn volume(&self) -> f64 {
        self.panjang * self.lebar * self.tinggi
    }
    fn luas_permukaan(&self) -> f64 {
        2.0 * (self.panjang * self.lebar + self.panjang * self.tinggi + self.lebar * self.tinggi)
    }
    fn diagonal_ruang(&self) -> f64 {
        (self.panjang.powi(2) + self.lebar.powi(2) + self.tinggi.powi(2)).sqrt()
    }
}
```

### c. `Bola`
```rust
struct Bola {
    r: f64,
}

impl Bola {
    fn new(r: f64) -> Self {
        Self { r }
    }
    fn luas_permukaan(&self) -> f64 {
        4.0 * PI * self.r.powi(2)
    }
    fn volume(&self) -> f64 {
        (4.0 / 3.0) * PI * self.r.powi(3)
    }
}
```

---


