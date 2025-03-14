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

| Normal         | Super         |
|---------------|--------------|
| `tambah()`    | `super_tambah()` |
| `kurang()`    | `super_kurang()` |
| `kali()`      | `super_kali()` |
| `bagi()`      | `super_bagi()` |
| `modulo()`    | - |
| `pangkat()`   | - |
| `akar_kuadrat()` | - |

#### Normal
Metode normal hanya menerima maksimal dua parameter.

| Metode | Parameter | Tipe Return | Hasil |
|--------|----------|-------------|---------|
| `tambah()` | `a: T, b: T` | `T` | `a + b` |
| `kurang()` | `a: T, b: T` | `T` | `a - b` |
| `kali()` | `a: T, b: T` | `T` | `a * b` |
| `bagi()` | `a: T, b: T` | `T` | `a / b` |
| `modulo()` | `a: T, b: T` | `T` | `a % b` |
| `pangkat()` | `base: T, exp: u32` | `T` | `base^exp` |
| `akar_kuadrat()` | `x: f64` | `f64` | `√x` |

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

| Metode | Parameter | Tipe Return | Hasil |
|--------|----------|-------------|---------|
| `super_tambah()` | `a: &[T]` | `T` | `a + b + c + ...` |
| `super_kurang()` | `a: &[T]` | `T` | `a - b - c - ...` |
| `super_kali()` | `a: &[T]` | `T` | `a * b * c * ...` |
| `super_bagi()` | `a: &[T]` | `T` | `a / b / c / ...` |

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
struct Aljabar;
Aljabar::splsv(a: f64, b: f64) -> Option<f64>
```

Contoh penggunaan:
```rust
use matematika_rs::sistem::aljabar::*;

fn main() {
    let x = Aljabar::splsv(6.0, 12.0);  // x = -12 / 6 = -2
    println!("{}", x.unwrap());
}
```

#### Sistem Persamaan Linear Dua Variabel (SPLDV)
Sintaks:
```rust
struct Aljabar;
Aljabar::spldv(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> Option<(f64, f64)>
```

Contoh penggunaan:
```rust
use matematika_rs::sistem::aljabar::*;

fn main() {
    let hasil = Aljabar::spldv(4.0, -3.0, 18.0, 3.0, 1.0, 7.0);
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

