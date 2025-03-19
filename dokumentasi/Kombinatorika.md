# Kombinatorika

Dokumentasi berisi penjelasan dari modul [`kombinatorika.rs`](https://github.com/lordpaijo/matematika.rs/blob/master/src/operasi/kombinatorika.rs).

---

## 📍 **Fungsi Dasar**

Fungsi dasar untuk menghitung **faktorial** dan **kombinasi**.

| Fungsi                             | Deskripsi                                              |
| ---------------------------------- | ------------------------------------------------------ |
| `faktorial(n: u64) -> u64`         | Menghitung faktorial dari `n` (\(n!\))                 |
| `kombinasi(n: u64, k: u64) -> u64` | Menghitung kombinasi \(C(n, k) = \frac{n!}{k!(n-k)!}\) |

```rust
pub fn faktorial(n: u64) -> u64 {
    (1..=n).product()
}

pub fn kombinasi(n: u64, k: u64) -> u64 {
    if k > n { return 0; }
    faktorial(n) / (faktorial(k) * faktorial(n - k))
}
```

---

## 📍 **Peluang**

Sub-modul `Peluang` berisi implementasi peluang menggunakan **dadu, koin, dan kantong kelereng**.

```rust
pub mod Peluang {
```

### 🎲 Dadu

Mewakili dadu standar **(angka 1-6)** dan peluangnya.

```rust
        #[derive(Debug)]
        pub struct Dadu {
            pub angka: [i64; 6],
        }

        impl Dadu {
            pub fn new() -> Self {
                Self { angka: [1, 2, 3, 4, 5, 6] }
            }

            pub fn muncul_angka(&self, target: i64) -> f64 {
                if self.angka.contains(&target) {
                    1.0 / self.angka.len() as f64
                } else { 0.0 }
            }
        }
```

---

### 🪙 Koin

Mewakili koin dengan **dua sisi (Angka & Gambar)**.

```rust
    #[derive(Debug)]
    pub struct Koin {
        pub sisi: [char; 2],
    }

    impl Koin {
        pub fn new() -> Self {
            Self { sisi: ['A', 'G'] } /* A = Angka, G = Gambar */
        }
    }
```

### 🔴⚪ Kantong Kelereng

Mewakili **kantong kelereng** dengan **warna merah dan putih**.

```rust
    #[derive(Debug)]
    pub struct KantongKelereng {
        pub merah: u32,
        pub putih: u32,
    }

    impl KantongKelereng {
        pub fn new(merah: u32, putih: u32) -> Self {
            Self { merah, putih }
        }
    }
}
```

### 🔹 **Contoh Penggunaan**

```rust
use matematika_rs::kombinatorika::peluang::{Dadu, Koin, KantongKelereng};

fn main() {
    let dadu = Dadu::new();
    println!("Peluang muncul angka 3: {}", dadu.muncul_angka(3));

    let koin = Koin::new();
    println!("Peluang muncul simbol: {}", koin.sisi[0]);

    let kantong = KantongKelereng::new(3, 4);
    println!("Peluang mengambil kelereng merah: {}", kantong.merah as f64 / (kantong.merah + kantong.putih) as f64);
}
```


