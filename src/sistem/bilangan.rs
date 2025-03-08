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
