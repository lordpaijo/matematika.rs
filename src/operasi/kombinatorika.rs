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
    
    #[derive(Debug)]
    pub struct Koin 
    {
        pub sisi: [char; 2],
    }

    impl Koin 
    {
        pub fn new() -> Self 
        {
            Self { sisi: ['A', 'G'], /* A = Angka, G = Gambar */ }
        }

        pub fn muncul(&self, target: char) -> f64 
        {
            if self.sisi.contains(&target) {
                1.0 / self.sisi.len() as f64
            } else { 0.0 }
        }

        pub fn muncul_beruntun(&self, target: char, jumlah: u32) -> f64 
        {
            if self.sisi.contains(&target) {
                (1.0 / self.sisi.len() as f64).powi(jumlah as i32)
            } else { 0.0 }
        }

        pub fn muncul_setidaknya_satu(&self, target: char, jumlah: u32) -> f64 
        {
            if self.sisi.contains(&target) 
            {
                let lawan = match target 
                {
                    'A' => 'G',
                    'G' => 'A',
                    _ => return 0.0, 
                };
                1.0 - self.muncul_beruntun(lawan, jumlah)
            } 
            else { 0.0 }
        }
    }

    #[derive(Debug)]
    pub struct KantongKelereng 
    {
        pub merah: u32,
        pub putih: u32,
    }

    impl KantongKelereng 
    {
        pub fn new(merah: u32, putih: u32) -> Self 
        {
            Self { merah, putih }
        }

        pub fn peluang_satu(&self, warna: char) -> f64 
        {
            let total = self.merah + self.putih;
            match warna {
                'M' => self.merah as f64 / total as f64,
                'P' => self.putih as f64 / total as f64,
                _ => 0.0, // Jika warna tidak valid
            }
        }

        pub fn peluang_dua_berurutan(&self, warna_pertama: char, warna_kedua: char) -> f64 
        {
            let total = self.merah + self.putih;

            let (jumlah_pertama, jumlah_kedua) = match (warna_pertama, warna_kedua) {
                ('M', 'M') => (self.merah, self.merah - 1),
                ('M', 'P') => (self.merah, self.putih),
                ('P', 'M') => (self.putih, self.merah),
                ('P', 'P') => (self.putih, self.putih - 1),
                _ => return 0.0, // Jika input tidak valid
            };

            (jumlah_pertama as f64 / total as f64) * (jumlah_kedua as f64 / (total - 1) as f64)
        }
    }
}
