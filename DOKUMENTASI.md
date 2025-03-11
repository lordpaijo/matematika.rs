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

Method-method di module ini ada dua macam, yaitu [normal](###) antara lain:

| Normal | Super |
|--------|-------|
| `tambah()` | `super_tambah()` |
| `kurang()` | `super_kurang()` |
| `kali()` | `super_kali()` |
| `bagi()` | `super_bagi()` |
| `modulo()` |  |
| `pangkat()` |  |
| `akar_kuadrat()` |  |

## Kombinatorika
