# Kombinatorika

Dokumentasi berisi penjelasan dari modul [`kombinatorika.rs`](https://github.com/lordpaijo/matematika.rs/blob/master/src/operasi/kombinatorika.rs).

---

## 📍 **Fungsi Dasar**

Fungsi dasar untuk menghitung **faktorial** dan **kombinasi**.

| Fungsi                             | Deskripsi                                              |
| ---------------------------------- | ------------------------------------------------------ |
| `faktorial(n: u64) -> u64`         | Menghitung faktorial dari `n` (\(n!\))                 |
| `kombinasi(n: u64, k: u64) -> u64` | Menghitung kombinasi \(C(n, k) = \frac{n!}{k!(n-k)!}\) |

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

Output:
```yaml
Faktorial dari 5 adalah: 120
Kombinasi C(5, 2) adalah: 10
```
 
---

## 📍 **Peluang**

Sub-modul `Peluang` berisi implementasi peluang menggunakan **dadu, koin, dan kantong kelereng**.

### 🎲 Dadu

Mewakili dadu standar **(angka 1-6)** dan peluangnya.

| Struktur           | Metode                         | Parameter                 | Tipe Return | Deskripsi |
|--------------------|--------------------------------|---------------------------|-------------|-----------|
| `Dadu`            | `new()`                        | -                         | `Dadu`      | Membuat objek dadu baru dengan angka 1-6 |
|                    | `muncul_angka()`               | `target: i64`             | `f64`       | Menghitung peluang munculnya angka tertentu |
|                    | `muncul_genap()`               | -                         | `f64`       | Menghitung peluang munculnya angka genap |
|                    | `muncul_lebih_dari()`          | `batas: i64`              | `f64`       | Menghitung peluang munculnya angka lebih dari nilai tertentu |
|                    | `muncul_kurang_dari()`         | `batas: i64`              | `f64`       | Menghitung peluang munculnya angka kurang dari nilai tertentu |

#### Contoh Penggunaan
```rust
use matematika-rs::operasi::kombinatorika::Peluang::*;

fn main() {
    let dadu = Dadu::new();
    println!("Peluang muncul angka 3: {}", dadu.muncul_angka(3));
}
```

### 🪙 Koin

Mewakili koin dengan **dua sisi (Angka & Gambar)**.

| Struktur           | Metode                         | Parameter                 | Tipe Return | Deskripsi |
|--------------------|--------------------------------|---------------------------|-------------|-----------|
| `Koin`            | `new()`                        | -                         | `Koin`      | Membuat objek koin baru dengan sisi 'A' (Angka) dan 'G' (Gambar) |
|                    | `muncul()`                     | `target: char`            | `f64`       | Menghitung peluang munculnya sisi tertentu |
|                    | `muncul_beruntun()`            | `target: char, jumlah: u32` | `f64`      | Menghitung peluang munculnya sisi tertentu secara beruntun |
|                    | `muncul_setidaknya_satu()`     | `target: char, jumlah: u32` | `f64`      | Menghitung peluang munculnya setidaknya satu sisi tertentu dalam beberapa kali lemparan |

#### Contoh Penggunaan
```rust
use matematika-rs::operasi::kombinatorika::Peluang::*;

fn main() {
    let koin = Koin::new();
    println!("Peluang muncul angka pada koin: {}", koin.muncul('A'));
}
```

### 🔴⚪ Kantong Kelereng

Mewakili **kantong kelereng** dengan **warna merah dan putih**.

| Struktur           | Metode                         | Parameter                 | Tipe Return | Deskripsi |
|--------------------|--------------------------------|---------------------------|-------------|-----------|
| `KantongKelereng` | `new()`                        | `merah: u32, putih: u32`  | `KantongKelereng` | Membuat objek kantong kelereng dengan jumlah kelereng merah dan putih |
|                    | `muncul_satu()`                | `warna: char`             | `f64`       | Menghitung peluang mengambil satu kelereng dengan warna tertentu |
|                    | `muncul_dua_berurutan()`       | `warna_pertama: char, warna_kedua: char` | `f64` | Menghitung peluang mengambil dua kelereng dengan warna tertentu secara berurutan |

#### Contoh Penggunaan
```rust
use peluang::Peluang::*;

fn main() {
    let kantong = KantongKelereng::new(5, 7);
    println!("Peluang mengambil kelereng merah: {}", kantong.muncul_satu('M'));
}
```

---
