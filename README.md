# matematika.rs

Sebuah crates / library matematika dasar untuk bahasa pemerograman rust yang ada karena developernya gabut, nolep, dan tidak berwibawa.

## Installasi

Tambahkan ke projek cargo-mu.
```sh
cargo add matematika-rs
```

Atau masukkan baris berikut kedalam bagian dependencies di Cargo.toml projekmu.
```toml
matematika-rs = "<Versi Crates>"
```

## Penggunaan

Layaknya crates / library rust pada umunya, pemakaian dimulai dari pemanggilan `matematika-rs`.

```rust
use matematika-rs;

fn main () 
{
    /* sebuah baris kode */
}
```

Untuk mengakses methods atau... apapun itulah, bisa dengan memanggil library diikuti dengan parent methods-nya.

```rust
use matematika-rs::operasi::aritmetika; // disini parentnya adalah operasi::aritmetika;
```

Baru deh kamu bisa memanggil method yang kamu pingin.

Contoh 1 (method aritmetika dasar) :
```rust
use matematika-rs::operasi::aritmetika;

fn main () 
{
    let x = 100; let y = 5;
    let z = [5, 6, 8, 125];
    let hasil = aritmetika::tambah(
                aritmetika::tambah(aritmetika::tambah(x, y), 
                                    aritmetika::kali(x, y)), 
                aritmetika::super_kurang(&z));
    
    println!("{}", hasil);
}
```
output:
```sh
471
```

Contoh 2 (method pembulatan, ganjil / genap, dan KPK & FPB) :
```rust
use matematika-rs::sistem::bilangan;
use matematika-rs::sistem::kelipatan;

fn main () 
{
    let x = 6.7487; let y: i64 = 8;
    let z: i64 = bilangan::bulat(x) as i64;
    bilangan::genap("cek", kelipatan::kpk(z, y));
    bilangan::ganjil("rubah", kelipatan::fpb(z, y));
}
```
