# Aljabar
Dokumentasi modul [`aljabar`](https://github.com/lordpaijo/matematika.rs/blob/master/src/sistem/aljabar.rs).

## Sistem Persamaan Linear
### Sistem Persamaan Linear Satu Variabel (SPLSV)
#### Sintaks:

```rust
struct Aljabar;

Aljabar::splsv(a: f64, b: f64) -> Option<f64>
        Some(-b / a)
```

Misalkan terdapat persamaan berikut:

*ax + b = 0*

Penyelesaiannya menggunakan rumus:

*x = −b / a​*

Implementasi dalam kode:

```rust
use matematika_rs::sistem::aljabar::*;

fn main() {
    let x = Aljabar::splsv(6.0, 12.0);  // x = -b / a
    println!("{}", x.unwrap());
    // x = -12 / 6 = -2
}
```

#### Proses Penyelesaian SPLSV dalam Kode
1. Fungsi `splsv(a, b)` menerima dua parameter, yaitu koefisien *a* dan konstanta *b*.

   ```rust
   pub fn splsv(a: f64, b: f64) -> Option<f64> 
   ```
   
3. Jika *a* bernilai nol, maka persamaan tidak memiliki solusi dan fungsi mengembalikan `None`.

   ```rust
   if a == 0.0 { return None; }
   ```
   
4. Jika *a* tidak nol, maka solusi dihitung menggunakan rumus *x = -b / a*.

   ```rust
   Some(-b / a)
   ```
   
5. Hasil dikembalikan dalam bentuk `Some(x)`.

### Sistem Persamaan Linear Dua Variabel (SPLDV)
#### Sintaks:
```rust
struct Aljabar;

Aljabar::spldv(
    a1: f64, b1: f64, c1: f64,
    a2: f64, b2: f64, c2: f64) -> Option<(f64, f64)>
        Some(x, y)
```
Misalkan terdapat sistem persamaan berikut:

*a₁x - b₁y = c₁*​

*a₂x + b₂y = c₂*

Penyelesaiannya dengan metode eliminasi:

*{ a₁x - b₁y = c₁ } . a₂*

*{ a₂x + b₂y = c₂ } . a₁*

Implementasi dalam kode:

```rust
use matematika_rs::sistem::aljabar::*;

fn main() {
    let hasil = Aljabar::spldv(
        4.0, -3.0, 18.0,  //  a₁x - b₁y = c₁
        3.0, 1.0, 7.0     //  a₂x + b₂y = c₂
    );
    println!("{:?}", hasil.unwrap());
}
```

Output:

```sh
(3.0, -2.0)
```

#### Proses Penyelesaian SPLDV dalam Kode
1. Fungsi `spldv(a1, b1, c1, a2, b2, c2)` menerima enam parameter, yaitu koefisien dan konstanta dari dua persamaan.

   ```rust
   pub fn spldv(
       a1: f64, b1: f64, c1: f64,
       a2: f64, b2: f64, c2: f64
   ) -> Option<(f64, f64)> 
   ```
   
3. Jika koefisien *a1* dan *a2* sama, maka persamaan tidak diubah.

   ```rust
   if a1 == a2 {
       new_b1 = b1; new_b2 = b2;
       new_c1 = c1; new_c2 = c2;
   ```
   
4. Jika berbeda, maka kedua persamaan dikalikan dengan koefisien masing-masing agar nilai *x* sama, sehingga dapat dieliminasi.

   ```rust
   new_b1 = b1 * a2; new_b2 = b2 * a1;
   new_c1 = c1 * a2; new_c2 = c2 * a1;
   ```
   
5. Setelah itu, dilakukan operasi penjumlahan atau pengurangan tergantung pada tanda koefisien *a1* dan *a2* untuk mengeliminasi *x*.

   ```rust
   if a1 < 0.00 || a2 < 0.00 {
       b3 = new_b1 + new_b2;
       c3 = new_c1 + new_c2;
   } else {
       b3 = new_b1 - new_b2;
       c3 = new_c1 - new_c2;
   }
   ```
   
6. Nilai *y* dihitung dengan *c3 / b3*.

   ```rust
   let y = c3 / b3;
   ```
   
7. Nilai *x* dihitung dengan mensubstitusi *y* ke dalam salah satu persamaan awal.

   ```rust
   let x = (c2 - (b2 * y)) / a2;
   ```
   
8. Hasil dikembalikan dalam bentuk `Some((x, y))`.

   ```rust
   Some((x, y))
   ```

