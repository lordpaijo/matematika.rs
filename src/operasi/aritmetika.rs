#![allow(dead_code)]

use std::f64;
use std::ops::*;
use std::collections::HashMap;

/* Normal (hanya dua angka) */

pub trait One { fn one() -> Self; }

impl One for i32 { fn one() -> Self { 1 } }
impl One for i64 { fn one() -> Self { 1 } }
impl One for f32 { fn one() -> Self { 1.0 } }
impl One for f64 { fn one() -> Self { 1.0 } }

// Jika exp == 0, maka akan mengembalikan nilai identitas (1).
pub fn pangkat<T>(base: T, exp: u32) -> T  where T: Mul<Output = T> + Copy + One,
{
    let mut hasil = T::one();
    for _ in 0..exp { hasil = hasil * base; }
    hasil
}

pub fn akar_kuadrat(x: f64) -> f64 
{
    if x < 0.0 { panic!("Akar kuadrat dari bilangan negatif tidak terdefinisi untuk f64."); }
    let mut z = x;
    for _ in 0..10 { z = (z + x / z) / 2.0; }
    z
}

pub fn tambah <T: Add<Output = T>> (a: T, b: T) -> T { a + b }
pub fn kurang <T: Sub<Output = T>> (a: T, b: T) -> T { a - b }
pub fn kali   <T: Mul<Output = T>> (a: T, b: T) -> T { a * b }
pub fn bagi   <T: Div<Output = T>> (a: T, b: T) -> T { a / b }
pub fn modulo <T: Rem<Output = T>> (a: T, b: T) -> T { a % b }

pub fn pangkat_optim<T> (mut base: T, mut exp: u32) -> T  
where T: Mul<Output = T> + Copy + One,
{
    let mut hasil = T::one();
    
    while exp > 0 {
        if exp % 2 == 1 { 
            hasil = hasil * base;
        }
        base = base * base;
        exp /= 2;  
    }

    hasil
}

pub fn pangkat_desimal (base: f64, exp: f64) -> f64 
{
    base.powf(exp)
}

pub fn akar_pangkat_n (x: f64, n: f64) -> f64 
{
    if x < 0.0 && n % 2.0 == 0.0 
    {
        panic!("Akar pangkat genap dari bilangan negatif tidak terdefinisi.");
    }
    x.powf(1.0 / n)
}

pub fn logaritma (x: f64, base: f64) -> f64 
{
    if x <= 0.0 || base <= 0.0 || base == 1.0 
    {
        panic!("Bilangan atau basis logaritma tidak valid.");
    }
    x.ln() / base.ln()
}

/* Super (Lebih dari dua angka, menggunakan array atau vector) */

pub fn super_tambah <T: Add<Output = T> + Copy> (angka: &[T]) -> T 
{ 
    let mut total = angka[0];
    for &num in &angka[1..] { total = total + num; }
    total 
}

pub fn super_kurang <T: Sub<Output = T> + Copy> (angka: &[T]) -> T 
{ 
    let mut total = angka[0];
    for &num in &angka[1..] { total = total - num; }
    total

}

pub fn super_kali <T: Mul<Output = T> + Copy> (angka: &[T]) -> T 
{ 
    let mut total = angka[0];
    for &num in &angka[1..] { total = total * num; }
    total 
  
}

pub fn super_bagi <T: Div<Output = T> + Copy> (angka: &[T]) -> T 
{ 
    let mut total = angka[0];
    for &num in &angka[1..] { total = total / num; }
    total 
  
}

pub fn super_logaritma (angka: &[f64], basis: f64) -> Vec<f64> 
{
    if basis <= 0.0 || basis == 1.0 
    {
        panic!("Basis logaritma harus lebih besar dari 0 dan tidak boleh 1.");
    }

    angka.iter().map(|&x| 
    {
        if x <= 0.0 
        {
            panic!("Logaritma tidak terdefinisi untuk angka <= 0.");
        }
        x.log(basis)
    }).collect()
}

pub fn mean (angka: &[f64]) -> f64 
{
    if angka.is_empty() 
    {
        panic!("Tidak bisa menghitung rata-rata dari array kosong.");
    }
    let total: f64 = angka.iter().sum();
    total / angka.len() as f64
}

pub fn median (angka: &mut [f64]) -> f64 
{
    if angka.is_empty() 
    {
        panic!("Tidak dapat menghitung median dari data kosong.");
    }
    
    angka.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = angka.len();
    
    if n % 2 == 1{ angka[n / 2] } 
    else { (angka[n / 2 - 1] + angka[n / 2]) / 2.0 }
}

pub fn modus (angka: &[i32]) -> Vec<i32> 
{
    if angka.is_empty() 
    {
        panic!("Tidak dapat menghitung modus dari data kosong.");
    }

    let mut frekuensi = HashMap::new();
    for &num in angka 
    {
        *frekuensi.entry(num).or_insert(0) += 1;
    }

    let max_freq = frekuensi.values().copied().max().unwrap();
    frekuensi
        .into_iter()
        .filter(|&(_, count)| count == max_freq)
        .map(|(num, _)| num)
        .collect()
}
