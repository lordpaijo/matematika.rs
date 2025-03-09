use std::ops::{Rem, Mul, Div};

pub trait Zero { fn zero() -> Self; }

impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }

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

