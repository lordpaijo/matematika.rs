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


pub fn absolut(x: f64) -> f64 
{
    x.abs()
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
