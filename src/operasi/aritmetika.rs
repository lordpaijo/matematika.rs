use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

/* ---- */

pub fn tambah <T: Add<Output = T>>(a: T, b: T) -> T { a + b }
pub fn kurang <T: Sub<Output = T>>(a: T, b: T) -> T { a - b }
pub fn kali   <T: Mul<Output = T>>(a: T, b: T) -> T { a * b }
pub fn bagi   <T: Div<Output = T>>(a: T, b: T) -> T { a / b }
