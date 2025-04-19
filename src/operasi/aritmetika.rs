use std::f64;
use std::ops::*;

/* Normal (hanya dua angka) */

pub trait Zero { fn zero() -> Self; }

impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }
impl Zero for f32 { fn zero() -> Self { 1.0 } }
impl Zero for f64 { fn zero() -> Self { 1.0 } }

pub trait One { fn one() -> Self; }

impl One for u32 { fn one() -> Self { 0 } }
impl One for u64 { fn one() -> Self { 0 } }
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

pub fn bulat (a: f64) -> f64
{
    let terpotong = a as i64;
    let pecahan = a - terpotong as f64;

    if a >= 0.0
    {
        if pecahan >= 0.5 { (terpotong + 1) as f64 }
        else { terpotong as f64 }
    }
    else
    {
        if pecahan >= 0.5 { (terpotong - 1) as f64 }
        else { terpotong as f64 }
    }
}

pub fn genap (mode: &str, mut a: i64) -> i64
{
    match mode
    {
        "cek" => {
            if a % 2 == 0 {
                println!("{} adalah bilangan genap.", a);
            } else {
                println!("{} bukanlah bilangan genap.", a);
            }
        }
        "rubah" => {
            if a % 2 != 0 {
                a += 1;
                println!("{}", a);
            } else {
                println!("{} sudah genap.", a);
            }
        }
        &_ => panic!("{} bukanlah sebuah mode!", mode),
    }
    a
}

pub fn ganjil (mode: &str, mut a: i64) -> i64
{
    match mode
    {
        "cek" => {
            if a % 2 != 0 {
                println!("{} adalah bilangan ganjil.", a);
            } else {
                println!("{} bukanlah bilangan ganjil.", a);
            }
        }
        "rubah" => {
            if a % 2 == 0 {
                a += 1;
                println!("{}", a);
            } else {
                println!("{} sudah ganjil.", a);
            }
        }
        &_ => panic!("{} bukanlah sebuah mode!", mode),
    }
    a
}


pub fn absolut (x: f64) -> f64
{
    x.abs()
}

pub fn fpb<T>(mut a: T, mut b: T) -> T where T: Copy + PartialEq + Rem<Output = T> + Zero,
{
    while b != T::zero() {
        let r = a % b;
        a = b; b = r; }
    a
}

pub fn kpk<T>(a: T, b: T) -> T
where T: Copy + PartialEq + Rem<Output = T> + Zero + Mul<Output = T> + Div<Output = T>,
{
    a * b / fpb(a, b)
}


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


pub fn super_genap (mode: &str, a: &mut [i64])
{
    match mode
    {
        "cek" => {
            for &num in a.iter() {
                if num % 2 == 0 {
                    println!("{} adalah bilangan genap.", num);
                } else {
                    println!("{} bukanlah bilangan genap.", num);
                }
            }
        }
        "rubah" => {
            for num in a.iter_mut() {
                if *num % 2 != 0 {
                    let temp = *num;
                    *num += 1;
                    println!("{} telah digenapkan menjadi: {}.", temp, *num);
                } else {
                    println!("{} sudah genap.", *num);
                }
            }
        }
        _ => panic!("{} bukanlah sebuah mode!", mode),
    }
}

pub fn super_ganjil (mode: &str, a: &mut [i64])
{
    match mode
    {
        "cek" => {
            for &num in a.iter() {
                if num % 2 != 0 {
                    println!("{} adalah bilangan ganjil.", num);
                } else {
                    println!("{} bukanlah bilangan ganjil.", num);
                }
            }
        }
        "rubah" => {
            for num in a.iter_mut() {
                if *num % 2 == 0 {
                    let temp = *num;
                    *num += 1;
                    println!("{} telah diganjilkan menjadi: {}.", temp, *num);
                } else {
                    println!("{} sudah ganjil.", *num);
                }
            }
        }
        _ => panic!("{} bukanlah sebuah mode!", mode),
    }
}

pub fn super_absolut (values: &[f64]) -> Vec<f64>
{
    values.iter().map(|&x| x.abs()).collect()
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

pub struct Fibonacci;

impl Fibonacci
{
    pub fn rekursif (n: u64) -> u64
    {
        if n <= 1 { return n; }
        Fibonacci::rekursif(n - 1) + Fibonacci::rekursif(n - 2)
    }

    pub fn iteratif (n: u64) -> u64
    {
        let (mut a, mut b) = (0, 1);
        for _ in 0..n
        {
            let temp = a;
            a = b;
            b = temp + b;
        }
        a
    }

    pub fn binet (n: u64) -> u64
    {
        let sqrt_5 = 5.0_f64.sqrt();
        let phi = (1.0 + sqrt_5) / 2.0;
        ((phi.powi(n as i32) / sqrt_5).round()) as u64
    }

    pub fn adalah_genap (n: u64) -> bool
    {
        Fibonacci::iteratif(n) % 2 == 0
    }

    pub fn adalah_prima (n: u64) -> bool
    {
        let num = Fibonacci::iteratif(n);
        if num < 2 { return false; }
        for i in 2..=((num as f64).sqrt() as u64)
        {
            if num % i == 0 { return false; }
        }
        true
    }
}
