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

Contoh 2 (method aljabar: sistem persamaan linear) :
```rust
use matematika_rs::sistem::aljabar::*;

fn main () 
{
    let a = 6.0; let b = 12.0;
    let x = Aljabar::splsv(a, b);
    let a1 = 3.0; let b1 = 2.0; let c1 = 12.0;
    let a2 = 5.0; let b2 = -1.0; let c2 = 4.0;
    let hasil = Aljabar::spldv(a1, b1, c1, a2, b2 , c2);
    println1("{}\n{:?}", x, hasil.unwrap());
}
```

output:
```sh
(-2.0)
(1.5384615384615383, 3.6923076923076925)
```

Contoh 3 (basis bilangan dan operasi aritmetika) :
```rust
use matematika_rs::operasi::aritmetika;
use matematika_rs::sistem::basis;

fn main ()
{
    let x: u64 = 680; let y: u64 = 87;
    let a = basis::konversi_basis(aritmetika::tambah(x ,y), 2);
    let b = basis::desimal_ke_biner(aritmetika::kali(x, y));
    let c = basis::biner_ke_hexadesimal(
            &basis::desimal_ke_biner(aritmetika::kurang(x, y)));
    let d = basis::hexadesimal_ke_oktal(
            &basis::desimal_ke_hexadesimal(aritmetika::bagi(x, y));
    println!("{}\n{}\n{}\n{}", a, b, c, d); 
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
