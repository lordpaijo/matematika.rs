#![allow(non_snake_case)]

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

pub mod Peluang
{
    #[derive(Debug)]
    pub struct Dadu 
    {
        pub angka: [i64; 6]
    }

    impl Dadu 
    {
        pub fn new() -> Self 
        {
            Self { angka: [1, 2, 3, 4, 5, 6], }
        }

        pub fn muncul_angka(&self, target: i64) -> f64 
        {
            if self.angka.contains(&target) {
                1.0 / self.angka.len() as f64
            } else { 0.0 }
        }

        pub fn muncul_genap(&self) -> f64 
        {
            let count = self.angka.iter().filter(|&&x| x % 2 == 0).count();
            count as f64 / self.angka.len() as f64
        }

        pub fn muncul_lebih_dari(&self, batas: i64) -> f64 
        {
            let count = self.angka.iter().filter(|&&x| x > batas).count();
            count as f64 / self.angka.len() as f64
        }

        pub fn muncul_kurang_dari(&self, batas: i64) -> f64 
        {
            let count = self.angka.iter().filter(|&&x| x < batas).count();
            count as f64 / self.angka.len() as f64
        }
    }

    pub struct Koin 
    {
        pub sisi: [char; 2],
    }

    impl Koin 
    {
        pub fn new() -> Self 
        {
            Self { sisi: ['H', 'T'], /* H = Kepala, T = Ekor */ }
        }

        pub fn peluang(&self, target: char) -> f64 
        {
            if self.sisi.contains(&target) {
                1.0 / self.sisi.len() as f64
            } else { 0.0 }
        }

        pub fn peluang_beruntun(&self, target: char, jumlah: u32) -> f64 
        {
            if self.sisi.contains(&target) {
                (1.0 / self.sisi.len() as f64).powi(jumlah as i32)
            } else { 0.0 }
        }

        pub fn peluang_setidaknya_satu(&self, target: char, jumlah: u32) -> f64 
        {
            if self.sisi.contains(&target) {
                1.0 - self.peluang_beruntun(if target == 'H' { 'T' } else { 'H' }, jumlah)
            } else { 0.0 }
        }
    }
}
