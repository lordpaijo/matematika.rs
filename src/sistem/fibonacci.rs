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
