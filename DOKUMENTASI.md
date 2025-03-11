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

Method-method di module ini ada dua macam, yaitu [normal](#Normal) antara lain:

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
    let x = tambah(64, 4); // 68
    let x = kurang(64, 4); // 60
    let x = kali(64, 4); // 256
    let x = bagi(64, 4); // 16
    let x = pangkat(64, 4); // 16777216
    let x = akar_kuadrat(64.0); // 8
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
    let x = super_tambah(&y); // 22
    let x = super_kurang(&y); // -10
    let x = super_kali(&y); // 840
    let x = super_bagi(&y); // 0.04
}
```

### Kombinatorika
