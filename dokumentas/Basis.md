# Konversi Basis

Modul [**Konversi Basis**](https://github.com/lordpaijo/matematika.rs/blob/master/src/sistem/basis.rs) pada pustaka **matematika.rs** menyediakan fungsi untuk mengubah angka dari satu basis ke basis lain. Modul ini mendukung basis antara 2 hingga 36.

## Fungsi Utama

### `konversi_basis`
#### Deskripsi
Fungsi ini mengonversi angka dari basis 10 ke basis yang ditentukan.

#### Sintaks
```rust
pub fn konversi_basis(num: u64, base: u32) -> String;
```

#### Parameter
- `num`: Angka dalam basis 10 yang akan dikonversi.
- `base`: Basis tujuan (antara 2 hingga 36).

#### Contoh Penggunaan
```rust
let hasil = konversi_basis(255, 16);
println!("{}", hasil); // Output: "FF"
```

---

### `parse_number`
#### Deskripsi
Fungsi ini mengubah angka dari basis tertentu ke basis 10.

#### Sintaks
```rust
pub fn parse_number(num_str: &str, from_base: u32) -> u64;
```

#### Parameter
- `num_str`: Angka dalam bentuk string.
- `from_base`: Basis asal angka.

#### Contoh Penggunaan
```rust
let hasil = parse_number("FF", 16);
println!("{}", hasil); // Output: 255
```

## Konversi Antar Basis
Modul ini juga menyediakan fungsi khusus untuk mengonversi angka antar basis umum: biner, oktal, desimal, dan heksadesimal.

### Dari Desimal
```rust
pub fn desimal_ke_biner(num: u64) -> String;
pub fn desimal_ke_oktal(num: u64) -> String;
pub fn desimal_ke_hexadesimal(num: u64) -> String;
```

### Dari Biner
```rust
pub fn biner_ke_desimal(num_str: &str) -> u64;
pub fn biner_ke_oktal(num_str: &str) -> String;
pub fn biner_ke_hexadesimal(num_str: &str) -> String;
```

### Dari Heksadesimal
```rust
pub fn hexadesimal_ke_desimal(num_str: &str) -> u64;
pub fn hexadesimal_ke_biner(num_str: &str) -> String;
pub fn hexadesimal_ke_oktal(num_str: &str) -> String;
```

### Dari Oktal
```rust
pub fn oktal_ke_desimal(num_str: &str) -> u64;
pub fn oktal_ke_biner(num_str: &str) -> String;
pub fn oktal_ke_hexadesimal(num_str: &str) -> String;
```

## Contoh Penggunaan

### Mengonversi Desimal ke Biner
```rust
let hasil = desimal_ke_biner(10);
println!("{}", hasil); // Output: "1010"
```

### Mengonversi Biner ke Desimal
```rust
let hasil = biner_ke_desimal("1010");
println!("{}", hasil); // Output: 10
```

### Mengonversi Heksadesimal ke Biner
```rust
let hasil = hexadesimal_ke_biner("FF");
println!("{}", hasil); // Output: "11111111"
```

### Mengonversi Oktal ke Desimal
```rust
let hasil = oktal_ke_desimal("17");
println!("{}", hasil); // Output: 15
```

---

Dokumentasi ini mencakup semua fungsi yang tersedia dalam modul Konversi Basis di pustaka **matematika.rs**. Semoga bermanfaat!

