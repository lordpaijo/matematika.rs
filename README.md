# matematika.rs

Sebuah crates / library matematika dasar user-friendly untuk bahasa pemerograman rust yang ada karena developernya gabut, nolep, dan tidak berwibawa.

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
use matematika_rs;

fn main () 
{
    /* sebuah baris kode */
}
```

Untuk mengakses methods atau... apapun itulah, bisa dengan memanggil library diikuti dengan parent methods-nya.

```rust
use matematika_rs::operasi::aritmetika; // disini parentnya adalah operasi::aritmetika;
```

Baru deh kamu bisa memanggil method yang kamu pingin.

Contoh 1 (method aritmetika dasar) :
```rust
use matematika_rs::operasi::aritmetika;

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
use matematika_rs::sistem::bilangan;
use matematika_rs::sistem::kelipatan;

fn main () 
{
    let x = 6.7487; let y: i64 = 8;
    let z: i64 = bilangan::bulat(x) as i64;
    bilangan::genap("cek", kelipatan::kpk(z, y));
    bilangan::ganjil("rubah", kelipatan::fpb(z, y));
}
```

output:
```sh
56 adalah bilangan genap.
1 sudah ganjil.
```

Contoh 3 (basis bilangan dan operasi aritmetika) :
```rust
use matematika_rs::operasi::aritmetika;
use matematika_rs::sistem::basis;

fn main ()
{
    let x: u64 = 680; let y: u64 = 87;
    println!("{}\n{}\n{}\n{}",
        basis::konversi_basis(aritmetika::tambah(x ,y), 2),
        basis::desimal_ke_biner(aritmetika::kali(x, y)),
        basis::biner_ke_hexadesimal(
           &basis::desimal_ke_biner(aritmetika::kurang(x, y)) 
        ),
        basis::hexadesimal_ke_oktal(
            &basis::desimal_ke_hexadesimal(aritmetika::bagi(x, y))
        )
    ); 
}
```

output:
```sh
1011111111
1110011100011000
251
7
```

Masih under construction, jadi harap panik kalau banyak bug karena yang make library atau crate atau apapun ini udah pasti jadi kelinci percobaanku (itung-itung kalian dapet bagian kontribusi lah). Selemat mencoba........
