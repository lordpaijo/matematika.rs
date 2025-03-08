use std::ops::*;

/* Normal (hanya dua angka) */

pub trait One {
    fn one() -> Self;
}

impl One for i32 {
    fn one() -> Self { 1 }
}

impl One for i64 {
    fn one() -> Self { 1 }
}

impl One for f32 {
    fn one() -> Self { 1.0 }
}

impl One for f64 {
    fn one() -> Self { 1.0 }
}

// Jika exp == 0, maka akan mengembalikan nilai identitas (1).
pub fn pangkat<T>(base: T, exp: u32) -> T  where T: Mul<Output = T> + Copy + One,
{
    let mut hasil = T::one();
    for _ in 0..exp { hasil = hasil * base; }
    hasil
}

pub fn tambah <T: Add<Output = T>>(a: T, b: T) -> T { a + b }
pub fn kurang <T: Sub<Output = T>>(a: T, b: T) -> T { a - b }
pub fn kali   <T: Mul<Output = T>>(a: T, b: T) -> T { a * b }
pub fn bagi   <T: Div<Output = T>>(a: T, b: T) -> T { a / b }
pub fn modulo <T: Rem<Output = T>>(a: T, b: T) -> T { a % b }


/* Super (Lebih dari dua angka, menggunakan array atau vector) */

pub fn super_tambah <T: Add<Output = T> + Copy>(angka: &[T]) -> T 
{ 
    let mut total = angka[0];
    for &num in &angka[1..] { total = total + num; }
    total 
}

pub fn super_kurang <T: Sub<Output = T> + Copy>(angka: &[T]) -> T 
{ 
    let mut total = angka[0];
    for &num in &angka[1..] { total = total - num; }
    total

}

pub fn super_kali <T: Mul<Output = T> + Copy>(angka: &[T]) -> T 
{ 
    let mut total = angka[0];
    for &num in &angka[1..] { total = total * num; }
    total 
  
}

pub fn super_bagi <T: Div<Output = T> + Copy>(angka: &[T]) -> T 
{ 
    let mut total = angka[0];
    for &num in &angka[1..] { total = total / num; }
    total 
  
}
