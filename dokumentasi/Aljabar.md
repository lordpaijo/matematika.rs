# Aljabar
Sebuah dokumentasi dari module [`alajabar`](https://github.com/lordpaijo/matematika.rs/blob/master/src/sistem/aljabar.rs)
## Sistem Persamaan Linear
### Sistem Persamaan Linear Satu Variable (SPLSV)
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

### Sistem Persamaan Linear Dua Variable (SPLDV)
Sintaks:
```rust
struct Aljabar;

Aljabar::splsv(
    a1: f64, b1: f64, c1: f64,
    a2: f64, b2: f64, c2: f64) -> Option<f64, f64>
        -> Some(x, y)
```

Misalkan kita punya permasalahan seperti ini:

*a1​x - b1​y = c1*​

*a2x + b2y = c2*

Dan penyelesaiannya adalah dengan cara mengeliminasi *x* seperti ini:

*{ a1x - b1y = c1 } . a2*

*{ a2x + b2y = c2 } . a1*

Maka penggunaan method `Aljabar::spldv()` adalah sebagai berikut:

```rust
use matematika_rs::sistem::aljabar::*;

fn main ()
{
    let hasil = Aljabar::spldv(
        4.0, -3.0, 18.0,  //  a1x - b1y = c1 
        3.0, 1.0, 7.0     //  a2x + b2y = c2
    );

    println!("{:?}", hasil.unwrap());
```

Dan outputnya adalah:
```sh
(3.0, -2.0)
```
