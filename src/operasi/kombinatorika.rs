#[allow(dead_code)]
pub fn faktorial(n: u64) -> u64 
{
    (1..=n).product()
}

#[allow(dead_code)]
pub fn kombinasi(n: u64, k: u64) -> u64 
{
    if k > n { return 0; }
    faktorial(n) / (faktorial(k) * faktorial(n - k))
}

