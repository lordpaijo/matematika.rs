# matematika.rs
Selamat datang di dokumentasi **matematika.rs**. Terima kasih telah berkunjung.

## Instalasi
Untuk menambahkan **matematika.rs** ke dalam proyek **Cargo** Anda, jalankan perintah berikut:

```sh
cargo add matematika-rs
```

Atau, dapat juga ditambahkan secara manual dengan menambahkan baris berikut ke dalam bagian `[dependencies]` di file `Cargo.toml`:

```toml
matematika-rs = "<Versi Crates>"
```

## Penggunaan
Mulailah dengan mengimpor pustaka ini:

```rust
use matematika_rs;

fn main() {
    // Contoh penggunaan
}
```

Untuk mengakses metode-metodenya, panggil pustaka ini diikuti dengan modul yang relevan.

```rust
use matematika_rs::operasi::aritmetika; // Modul yang digunakan adalah operasi::aritmetika
```

Secara umum, modul dalam **matematika.rs** dibagi menjadi dua kategori utama: [Operasi](#operasi) dan [Sistem](#sistem).

---

## Operasi
### Aritmetika
Modul ini berisi operasi aritmetika dasar yang terbagi menjadi dua jenis: **Normal** dan **Super**.

| Normal             | Super                  |
| ------------------ | ---------------------- |
| `tambah()`         | `super_tambah()`       |
| `kurang()`         | `super_kurang()`       |
| `kali()`           | `super_kali()`         |
| `bagi()`           | `super_bagi()`         |
| `modulo()`         | -                      |
| `pangkat()`        | -                      |
| `akar_kuadrat()`   | -                      |
| `akar_pangkat_n()` | -                      |
| `logaritma()`      | `super_logaritma()`    |

#### Normal
Metode normal hanya menerima maksimal dua parameter.

Berikut tabel yang sudah diperbarui dengan dua fungsi tambahan:  

| Metode              | Parameter            | Tipe Return | Hasil                        |
|---------------------|----------------------|-------------|-----------------------------|
| `tambah()`           | `a: T, b: T`         | `T`         | `a + b`                     |
| `kurang()`           | `a: T, b: T`         | `T`         | `a - b`                     |
| `kali()`             | `a: T, b: T`         | `T`         | `a * b`                     |
| `bagi()`             | `a: T, b: T`         | `T`         | `a / b`                     |
| `modulo()`           | `a: T, b: T`         | `T`         | `a % b`                     |
| `pangkat()`          | `base: T, exp: u32`  | `T`         | `base^exp`                  |
| `pangkat_optim()`    | `base: T, exp: u32`  | `T`         | `base^exp` (efisien)        |
| `pangkat_desimal()`  | `base: f64, exp: f64`| `f64`       | `base^exp` (desimal)        |
| `akar_kuadrat()`     | `x: f64`             | `f64`       | `√x`                        |
| `akar_pangkat_n()`   | `x: f64, n: f64`     | `f64`       | `x^(1/n)`                   |
| `logaritma()`        | `x: f64, base: f64`  | `f64`       | `log_base(x)`               |



Contoh:
```rust
use matematika_rs::operasi::aritmetika::*;

fn main() {
    let hasil = tambah(64, 4); // 64 + 4 = 68
    println!("{}", hasil);
}
```

#### Super
Metode super menerima satu parameter berupa array untuk melakukan operasi pada banyak angka sekaligus.

| Metode              | Parameter            | Tipe Return | Hasil                        |
|---------------------|----------------------|-------------|-----------------------------|
| `super_tambah()`     | `a: &[T]`            | `T`         | `a + b + c + ...`           |
| `super_kurang()`     | `a: &[T]`            | `T`         | `a - b - c - ...`           |
| `super_kali()`       | `a: &[T]`            | `T`         | `a * b * c * ...`           |
| `super_bagi()`       | `a: &[T]`            | `T`         | `a / b / c / ...`           |
| `super_logaritma()`  | `a: &[f64], base: f64` | `Vec<f64>` | `log_base(a1), log_base(a2), ...` |

Contoh:
```rust
use matematika_rs::operasi::aritmetika::*;

fn main() {
    let angka = [6.0, 4.0, 5.0, 7.0];
    let hasil = super_tambah(&angka); // 6 + 4 + 5 + 7 = 22
    println!("{}", hasil);
}
```

---

### Kombinatorika
Modul ini menyediakan fungsi untuk menghitung operasi kombinatorika dasar dan peluang kejadian dari berbagai objek probabilitas, termasuk dadu, koin, dan kantong kelereng.

#### Operasi Dasar
Fungsi dasar untuk menghitung **faktorial** dan **kombinasi**.

| Fungsi                                      | Deskripsi                                                                            |
|--------------------------------------------|-------------------------------------------------------------------------------------|
| `faktorial(n: u64) -> u64`                 | Menghitung faktorial dari `n` (\(n!\))                                           |
| `kombinasi(n: u64, k: u64) -> u64`         | Menghitung kombinasi \(C(n, k) = \frac{n!}{k!(n-k)!}\)                           |
| `permutasi(n: u64, r: u64) -> u64`         | Menghitung permutasi \(P(n, r) = \frac{n!}{(n-r)!}\)                             |
| `permutasi_perulangan(n: u64, &[u64]) -> u64` | Menghitung permutasi dengan pengulangan \(P(n; n_1, n_2, ..., n_k) = \frac{n!}{n_1! \cdot n_2! \cdot ... \cdot n_k!}\) |
| `kombinasi_perulangan(n: u64, r: u64) -> u64` | Menghitung kombinasi dengan pengulangan \(C'(n, r) = \frac{(n + r - 1)!}{r!(n - 1)!}\) |

Contoh:

```rust
use matematika_rs::kombinatorika::{faktorial, kombinasi};

fn main() {
    let n = 5;
    let k = 2;

    let hasil_faktorial = faktorial(n);
    println!("Faktorial dari {} adalah: {}", n, hasil_faktorial);

    let hasil_kombinasi = kombinasi(n, k);
    println!("Kombinasi C({}, {}) adalah: {}", n, k, hasil_kombinasi);
}
```

#### Peluang
| Struktur           | Metode                         | Parameter                 | Tipe Return | Deskripsi |
|--------------------|--------------------------------|---------------------------|-------------|-----------|
| `Dadu`            | `new()`                        | -                         | `Dadu`      | Membuat objek dadu baru dengan angka 1-6 |
|                    | `muncul_angka()`               | `target: i64`             | `f64`       | Menghitung peluang munculnya angka tertentu |
|                    | `muncul_genap()`               | -                         | `f64`       | Menghitung peluang munculnya angka genap |
|                    | `muncul_lebih_dari()`          | `batas: i64`              | `f64`       | Menghitung peluang munculnya angka lebih dari nilai tertentu |
|                    | `muncul_kurang_dari()`         | `batas: i64`              | `f64`       | Menghitung peluang munculnya angka kurang dari nilai tertentu |
| `Koin`            | `new()`                        | -                         | `Koin`      | Membuat objek koin baru dengan sisi 'A' (Angka) dan 'G' (Gambar) |
|                    | `muncul()`                     | `target: char`            | `f64`       | Menghitung peluang munculnya sisi tertentu |
|                    | `muncul_beruntun()`            | `target: char, jumlah: u32` | `f64`      | Menghitung peluang munculnya sisi tertentu secara beruntun |
|                    | `muncul_setidaknya_satu()`     | `target: char, jumlah: u32` | `f64`      | Menghitung peluang munculnya setidaknya satu sisi tertentu dalam beberapa kali lemparan |
| `KantongKelereng` | `new()`                        | `merah: u32, putih: u32`  | `KantongKelereng` | Membuat objek kantong kelereng dengan jumlah kelereng merah dan putih |
|                    | `muncul_satu()`                | `warna: char`             | `f64`       | Menghitung peluang mengambil satu kelereng dengan warna tertentu |
|                    | `muncul_dua_berurutan()`       | `warna_pertama: char, warna_kedua: char` | `f64` | Menghitung peluang mengambil dua kelereng dengan warna tertentu secara berurutan |

Contoh:
```rust
use peluang::Peluang::*;

fn main() {
    let dadu = Dadu::new();
    println!("Peluang muncul angka 3: {}", dadu.muncul_angka(3));

    let koin = Koin::new();
    println!("Peluang muncul angka pada koin: {}", koin.muncul('A'));

    let kantong = KantongKelereng::new(5, 7);
    println!("Peluang mengambil kelereng merah: {}", kantong.muncul_satu('M'));
}
```

### Statistika
Modul ini berisi metode-metode hitung dari materi statistika.

| Metod | Parameter | Tipe Return | Hasil |
|-------|-----------|-------------|-------|
| `mean()`             | `a: &[f64]`         | `f64`       | `Σa / n` (rata-rata)        |
| `median()`           | `a: &[f64]`         | `f64`       | Nilai tengah.                |
| `modus()`            | `a: &[f64]`         | `Vec<f64>`  | Nilai yang paling sering muncul.    |

Contoh:
```rust
use matematika_rs::operasi::statistika::*;

fn main ()
{
    let x = [4.00, 5.00, 5.00, 6.00 7.00, 3.00, 3.00, 5.00, 4.00 6.00];
    let a = mean(&x);
    let b = median(&x);
    let c = modus(&x);
}
```

---

## Sistem
### Pembulatan dan Bilangan Ganjil-Genap
Modul ini menyediakan fungsi untuk melakukan pembulatan serta pengecekan dan perubahan bilangan ganjil-genap.

| Fungsi | Parameter | Tipe Return | Deskripsi |
|--------|----------|-------------|-----------|
| `bulat()` | `a: f64` | `f64` | Membulatkan angka ke bilangan bulat terdekat |
| `genap("cek", a)` | `a: i64` | `i64` | Mengecek apakah `a` adalah bilangan genap |
| `genap("rubah", a)` | `a: i64` | `i64` | Mengubah `a` menjadi bilangan genap terdekat |
| `ganjil("cek", a)` | `a: i64` | `i64` | Mengecek apakah `a` adalah bilangan ganjil |
| `ganjil("rubah", a)` | `a: i64` | `i64` | Mengubah `a` menjadi bilangan ganjil terdekat |
| `super_genap("cek", &[i64])` | `a: &mut [i64]` | - | Mengecek apakah elemen dalam array adalah bilangan genap |
| `super_genap("rubah", &[i64])` | `a: &mut [i64]` | - | Mengubah elemen dalam array menjadi bilangan genap |
| `super_ganjil("cek", &[i64])` | `a: &mut [i64]` | - | Mengecek apakah elemen dalam array adalah bilangan ganjil |
| `super_ganjil("rubah", &[i64])` | `a: &mut [i64]` | - | Mengubah elemen dalam array menjadi bilangan ganjil |

Contoh penggunaan:

```rust
use matematika_rs::sistem::*;

fn main() {
    let angka = 4.7;
    println!("{}", bulat(angka)); // Output: 5.0

    let mut nilai = 7;
    println!("{}", genap("rubah", nilai)); // Output: 8
}
```

```rust
use matematika_rs::sistem::*;

fn main() {
    let mut angka = [3, 6, 9, 12];
    super_genap("rubah", &mut angka);
    println!("{:?}", angka); // Output: [4, 6, 10, 12]
}
```

### Faktor Persekutuan Terbesar (FPB) & Kelipatan Persekutuan Terkecil (KPK)
Modul ini menyediakan fungsi untuk mencari FPB dan KPK dari dua bilangan.

| Fungsi | Parameter | Tipe Return | Deskripsi |
|--------|----------|-------------|-----------|
| `fpb(a, b)` | `a: T, b: T` | `T` | Menghitung faktor persekutuan terbesar dari `a` dan `b` |
| `kpk(a, b)` | `a: T, b: T` | `T` | Menghitung kelipatan persekutuan terkecil dari `a` dan `b` |

```rust
use matematika_rs::sistem::*;

fn main() {
    let a = 36;
    let b = 48;
    println!("FPB: {}", fpb(a, b)); // Output: 12
    println!("KPK: {}", kpk(a, b)); // Output: 144
}
```

### Geometri

Modul ini menyediakan berbagai struktur dan metode untuk menghitung properti bangun datar dan bangun ruang, seperti luas, keliling, dan volume.

## 1. `bangun_datar`
Modul ini berisi struktur untuk bangun datar seperti persegi dan lingkaran, masing-masing dengan metode untuk menghitung luas dan keliling.

### Contoh:
#### `Persegi`
```rust
struct Persegi {
    sisi: f64,
}

impl Persegi {
    fn luas(&self) -> f64 {
        self.sisi.powi(2)
    }
    fn keliling(&self) -> f64 {
        4.0 * self.sisi
    }
}
```

## 2. `bangun_ruang`
Modul ini mencakup struktur bangun ruang seperti kubus dan bola, dengan metode untuk menghitung volume dan luas permukaan.

### Contoh:
#### `Kubus`
```rust
struct Kubus {
    sisi: f64,
}

impl Kubus {
    fn volume(&self) -> f64 {
        self.sisi.powi(3)
    }
    fn luas_permukaan(&self) -> f64 {
        6.0 * self.sisi.powi(2)
    }
}
```

## 3. `Trigonometri`
Struktur ini mencakup metode-metode hitung dari materi trigonometri.

| Metode              | Parameter            | Tipe Return | Hasil                        |
|---------------------|----------------------|-------------|-----------------------------|
| `sin_deg()`          | `deg: T`             | `T`         | Nilai sinus dari sudut dalam derajat |
| `cos_deg()`          | `deg: T`             | `T`         | Nilai cosinus dari sudut dalam derajat |
| `tan_deg()`          | `deg: T`             | `T`         | Nilai tangen dari sudut dalam derajat |

---

Contoh:
```rust
use matematika::trigonometri::{sin_deg, cos_deg, tan_deg};

fn main() {
    let angle_f64: f64 = 45.0;
    let angle_f32: f32 = 30.0;

    println!("sin 45° (f64): {}", sin_deg(angle_f64));
    println!("cos 30° (f32): {}", cos_deg(angle_f32));
    println!("tan 45° (f64): {}", tan_deg(angle_f64));
}
```

### Deret Fibonacci
Modul ini menyediakan metode untuk menghitung deret Fibonacci dengan berbagai pendekatan.

| Fungsi | Parameter | Tipe Return | Deskripsi |
|--------|----------|-------------|-----------|
| `rekursif(n)` | `n: u64` | `u64` | Menghitung bilangan Fibonacci ke-`n` dengan rekursi |
| `iteratif(n)` | `n: u64` | `u64` | Menghitung bilangan Fibonacci ke-`n` dengan iterasi |
| `binet(n)` | `n: u64` | `u64` | Menghitung bilangan Fibonacci ke-`n` menggunakan rumus Binet |
| `adalah_genap(n)` | `n: u64` | `bool` | Mengecek apakah bilangan Fibonacci ke-`n` adalah genap |
| `adalah_prima(n)` | `n: u64` | `bool` | Mengecek apakah bilangan Fibonacci ke-`n` adalah bilangan prima |

Contoh penggunaan:

```rust
use matematika_rs::sistem::fibonacci::*;

fn main() {
    println!("Fibonacci ke-10: {}", Fibonacci::iteratif(10));
    println!("Apakah Fibonacci ke-10 genap? {}", Fibonacci::adalah_genap(10));
    println!("Apakah Fibonacci ke-10 prima? {}", Fibonacci::adalah_prima(10));
}
```

### Aljabar
Modul **Aljabar** berisi metode untuk menyelesaikan sistem persamaan linear.

#### Sistem Persamaan Linear Satu Variabel (SPLSV)
Sintaks:
```rust
struct SistemPersamaan;
SistemPersamaan::splsv(a: f64, b: f64) -> Option<f64>
```

Contoh penggunaan:
```rust
use matematika_rs::sistem::aljabar::*;

fn main() {
    let x = SistemPersamaan::splsv(6.0, 12.0);  // x = -12 / 6 = -2
    println!("{}", x.unwrap());
}
```

#### Sistem Persamaan Linear Dua Variabel (SPLDV)
Sintaks:
```rust
struct SistemPersamaan;
SistemPersamaan::spldv(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> Option<(f64, f64)>
```

Contoh penggunaan:
```rust
use matematika_rs::sistem::aljabar::*;

fn main() {
    let hasil = SistemPersamaan::spldv(4.0, -3.0, 18.0, 3.0, 1.0, 7.0);
    println!("{:?}", hasil.unwrap()); // Output: (3.0, -2.0)
}
```

### Konversi Basis
Modul ini menyediakan fungsi untuk mengonversi angka antara berbagai basis.

#### Fungsi Umum
```rust
pub fn konversi_basis(num: u64, base: u32) -> String;
pub fn parse_number(num_str: &str, from_base: u32) -> u64;
```

#### Konversi Antar Basis
```rust
pub fn desimal_ke_biner(num: u64) -> String;
pub fn desimal_ke_oktal(num: u64) -> String;
pub fn desimal_ke_hexadesimal(num: u64) -> String;

pub fn biner_ke_desimal(num_str: &str) -> u64;
pub fn biner_ke_oktal(num_str: &str) -> String;
pub fn biner_ke_hexadesimal(num_str: &str) -> String;

pub fn hexadesimal_ke_desimal(num_str: &str) -> u64;
pub fn hexadesimal_ke_biner(num_str: &str) -> String;
pub fn hexadesimal_ke_oktal(num_str: &str) -> String;

pub fn oktal_ke_desimal(num_str: &str) -> u64;
pub fn oktal_ke_biner(num_str: &str) -> String;
pub fn oktal_ke_hexadesimal(num_str: &str) -> String;
```

Untuk detail lebih lanjut, silakan merujuk pada [dokumentasi lengkap](https://github.com/lordpaijo/matematika.rs/blob/master/dokumentasi/).

---

## Penutup
blukutupblukutupblukutup

