use std::ops::*;

/* Normal (hanya dua angka) */

// Tanpa tipe data khusus
pub fn tambah <T: Add<Output = T>>(a: T, b: T) -> T { a + b }
pub fn kurang <T: Sub<Output = T>>(a: T, b: T) -> T { a - b }
pub fn kali   <T: Mul<Output = T>>(a: T, b: T) -> T { a * b }
pub fn bagi   <T: Div<Output = T>>(a: T, b: T) -> T { a / b }
pub fn modulo <T: Rem<Output = T>>(a: T, b: T) -> T { a % b }



/* Super (Lebih dari dua angka) */

// Tanpa tipe data khusus
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
