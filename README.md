
> [!WARNING]
> 
> REPO INI SERING MENDAPATKAN UPDATE.
> 
> KESTABILAN DI DALAM *BRANCH* [`master`](https://github.com/lordpaijo/matematika.rs/tree/master)
> 
> TIDAK DAPAT DIPASTIKAN!

# matematika.rs

`matematika.rs` adalah sebuah pustaka (crate) Rust yang menyediakan berbagai fungsi matematika dasar dengan antarmuka yang mudah digunakan.

## Instalasi

Tambahkan pustaka ini ke dalam proyek Cargo Anda dengan perintah berikut:

```sh
cargo add matematika-rs
```

Atau tambahkan secara manual ke dalam bagian `dependencies` pada file `Cargo.toml`:

```toml
matematika-rs = "<Versi Crates>"
```

## Penggunaan

Untuk menggunakan pustaka ini, import terlebih dahulu `matematika-rs` ke dalam kode Anda:

```rust
use matematika_rs;

fn main() {
    // Implementasi kode di sini
}
```

Untuk mengakses metode yang tersedia, panggil pustaka diikuti dengan modul yang bersangkutan:

```rust
use matematika_rs::operasi::aritmetika; // Modul operasi aritmetika
```

### Contoh Penggunaan

#### 1. Operasi Aritmetika Dasar

```rust
use matematika_rs::operasi::aritmetika;

fn main() {
    let x = 100;
    let y = 5;
    let z = [5, 6, 8, 125];
    let hasil = aritmetika::tambah(
        aritmetika::tambah(
            aritmetika::tambah(x, y),
            aritmetika::kali(x, y)
        ),
        aritmetika::super_kurang(&z)
    );
    println!("{}", hasil);
}
```

**Output:**
```sh
471
```

#### 2. Sistem Persamaan Linear

```rust
use matematika_rs::sistem::aljabar::*;

fn main() {
    let a = 6.0; 
    let b = 12.0;
    let x = SistemPersamaan::splsv(a, b);

    let a1 = 4.0; let b1 = -3.0; let c1 = 18.0;
    let a2 = 3.0; let b2 = 1.0; let c2 = 7.0;
    let hasil = SistemPersamaan::spldv(a1, b1, c1, a2, b2, c2);

    println!("{}\n{:?}", x, hasil.unwrap());
}
```

**Output:**
```sh
(-2.0)
(3.0, -2.0)
```

#### 3. Konversi Basis Bilangan dan Operasi Aritmetika

```rust
use matematika_rs::operasi::aritmetika;
use matematika_rs::sistem::basis;

fn main() {
    let x: u64 = 680;
    let y: u64 = 87;
    let a = basis::konversi_basis(aritmetika::tambah(x, y), 2);
    let b = basis::desimal_ke_biner(aritmetika::kali(x, y));
    let c = basis::biner_ke_hexadesimal(
        &basis::desimal_ke_biner(aritmetika::kurang(x, y))
    );
    let d = basis::hexadesimal_ke_oktal(
        &basis::desimal_ke_hexadesimal(aritmetika::bagi(x, y))
    );
    println!("{}\n{}\n{}\n{}", a, b, c, d);
}
```

**Output:**
```sh
1011111111
1110011100011000
251
7
```

## Dokumentasi Lengkap

Dokumentasi lengkap pustaka ini dapat diakses melalui tautan berikut:
[Dokumentasi matematika.rs](https://github.com/lordpaijo/matematika.rs/blob/master/DOKUMENTASI.md).

## Status Pengembangan

Pustaka ini masih dalam tahap pengembangan aktif, sehingga mungkin terdapat bug atau keterbatasan dalam fungsionalitasnya. Pengguna diharapkan untuk memberikan masukan dan kontribusi guna meningkatkan kualitas pustaka ini.

Selamat mencoba!

