# matematika.rs
Ini adalah sebuah dokumentasi, terimaksih.

## Installasi
Penambahan ke projek cargo-mu melalui `cargo add`.

```sh
cargo add matematika-rs
```

Atau dengan menambahkan baris berikut kedalam bagian `dependencies` di config `Cargo.toml` projekmu.

```toml
matematika-rs = "<Versi Crates>"
```

## Penggunaan
Penggunaan library ini dimulai dari pemanggilan `matematika-rs`.

```rust
use matematika_rs;

fn main ()
{
    /* 
        Sebuah baris kode
    */
}
```

Dan untuk mengakses method-methodnya bisa dengan memanggil library diikuti dengan module (parent) dari method-method bersangkutan.

```rust
use matematika_rs::operasi::aritmetika; // disini module-nya adalah operasi::aritmetika
```

Module-module `matematika-rs` dikelompokkan menjadi dua, yaitu [operasi](#Operasi) dan [sistem](#Sistem).

## Operasi
### Aritmetika 
Module ini adalah module yang berisikan operasi aritmetika dasar.

Method-method di module ini ada dua macam, yaitu [normal](#Normal) dan [super](#Super).

| Normal | Super |
|--------|-------|
| `tambah()` | `super_tambah()` |
| `kurang()` | `super_kurang()` |
| `kali()` | `super_kali()` |
| `bagi()` | `super_bagi()` |
| `modulo()` |  |
| `pangkat()` |  |
| `akar_kuadrat()` |  |

#### Normal
Method-method normal adalah method yang hanya mengambil tidak lebih dari dua parameter.
| Method | Parameter | Tipe Return | Return |
|:-------|:---------:|:-----------:|-------:|
| `tambah()` | `a: T, b: T` | `T` | `a + b` |
| `kurang()` | `a: T, b: T` | `T` | `a - b` |
| `kali()` | `a: T, b: T` | `T` | `a * b` |
| `bagi()` | `a: T, b: T` | `T` | `a / b` | 
| `modulo()` | `a: T, b: T` | `T` | `a % b` |
| `pangkat()` | `base: T, exp: u32` | `T` | `hasil * base` |
| `akar_kuadrat()` | `x: f64` | `f64` | `(z + x / z) / 2.0` |

Contoh:
```rust
use matematika_rs::operasi::aritmetika::*;

fn main ()
{
    let x = tambah(64, 4); // 64 + 4 = 68
    let x = kurang(64, 4); // 64 - 4 = 60
    let x = kali(64, 4); // 64 * 4 = 256
    let x = bagi(64, 4); // 64 / 4 = 16
    let x = pangkat(64, 4); // 64 ^ 4 = 16777216
    let x = akar_kuadrat(64.0); // √64 = 8
}
```

#### Super
Method-method super adalah method yang parameternya hanya satu tapi bisa mengambil lebih dari dua nilai. (Yes pake array)
| Method | Parameter | Tipe Return | Return |
|:-------|:---------:|:-----------:|-------:|
| `super_tambah()` | `a: &[T]` | `T` | `a + b + c + d + ...` |
| `super_kurang()` | `a: &[T]` | `T` | `a - b - c - d - ...` |
| `super_kali()` | `a: &[T]` | `T` | `a * b * c * d * ...` |
| `super_bagi()` | `a: &[T]` | `T` | `a / b / c / d / ...` |

Contoh:
```rust
use matematika_rs::operasi::aritmetika::*;

fn main ()
{
    let y = [6.0, 4.0, 5.0 , 7.0];
    let x = super_tambah(&y); // 6 + 4 + 5 + 7 = 22
    let x = super_kurang(&y); // 6 - 4 - 5 - 7 = -10
    let x = super_kali(&y); // 6 * 4 * 5 * 7 = 840
    let x = super_bagi(&y); // 6 / 4 / 5 / 7 = 0.04
}
```

## Sistem
### Aljabar
Module ini berisi sistem Aljabar (`struct`) (untuk sekarang masih tingkat smp karena saya masih smp) dan operasinya seperti: Sistem Persamaan Linear Satu Variable (SPLSV), Sistem Persamaan Linear Dua Variable (SPLDV), Sistem Persamaan Linear Tiga Variable (SPLTV).

#### Sistem Persamaan Linear Satu Variable (SPLSV)
Sintaks:
```rust
struct Aljabar;

Aljabar::splsv(a: f64, b: f64) -> Option<f64>
        -> Some(-b / a)
```

Misalkan kita punya permasalahan seperti ini:

*ax + b = 0*

Dan penyelesaiannya adalah dengan rumus seperti ini:

*x = −b / a​*

Maka penggunaan method `Aljabar::splsv()` adalah sebagai berikut:
```rust
use matematika_rs::sistem::aljabar::*;

fn main ()
{
    //
    // ax + b = 0
    //
    let x = Aljabar::splsv(6.0, 12.0);  // x = -b / a
    println!("{}", x);
    //
    // x = -12 / 6 = -2
    //
}
```

#### Sistem Persamaan Linear Dua Variable (SPLDV)
Sintaks:
```rust
struct Aljabar;

Aljabar::splsv(
    a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> Option<f64, f64>
        -> Some(x, y)
```

Misalkan kita punya permasalahan seperti ini:

*a1​x + b1​y = c1*​

*a2x + b2y = c2*

Dan penyelesaiannya adalah dengan rumus seperti ini:

*x = (a1 . ​b2 ​− a2 . ​b1) / ​(c1 . ​b2 ​− c2 . ​b1)​​*        *Gampangannya kita*

*y = (a1 . ​b2 ​− a2​ . b1)​ / (a1​ . c2​ − a2 . ​c1)*        *kali silang.*  

dan jika determinannya nol (yes, pake determinan, komputer lebih anteng kalo gini), maka tidak membuahkan hasil.


Maka penggunaan method `Aljabar::spldv()` adalah sebagai berikut:

```rust
use matematika_rs::sistem::aljabar::*;

fn main ()
{
    //                         a1   b1   c1    a2    b2   c2
    let hasil = Aljabar::spldv(3.0, 2.0, 12.0, 5.0, -1.0, 4.0);
    
    match hasil {
        Some((x, y)) => println!("x = {}, y = {}", x, y),
        None => println!("Tidak ada solusi"),
        //
        // Hasil ekspektasi -> x = 1.538, y = 3.692 (Iya, ekspektasi, komputernya sering ngambek jadi cuma bisa berekspektasi.)
        //
    }
}
```

## Penutup
Cukup sekian dulu, malas nulis dan besok ada ujian.
