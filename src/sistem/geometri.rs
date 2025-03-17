pub mod bangun_datar 
{ 
    pub struct Persegi
    {
        sisi: f64,
    }

    impl Persegi
    {
        pub fn new (sisi: f64) -> Self 
        {
            Self { sisi }
        }

        pub fn luas (&self) -> f64 
        {
            self.sisi.powf(2.00)
        }

        pub fn keliling (&self) -> f64
        {
            self.sisi * 4.00 
        }
    }
    
    pub struct PersegiPanjang
    {
        panjang: f64, lebar: f64,
    }

    impl PersegiPanjang 
    {
        pub fn new (panjang: f64, lebar: f64) -> Self
        {
            Self { panjang, lebar }
        }

        pub fn luas (&self) -> f64
        {
            self.panjang * self.lebar
        }

        pub fn keliling (&self) -> f64
        {
            (self.panjang + self.lebar) * 2.00 
        }
    }

    pub struct Segitiga
    {
        alas: f64, tinggi: f64, sisi: [f64; 3]
    }

    impl Segitiga
    {
        pub fn new (alas: f64, tinggi: f64, sisi: [f64; 3]) -> Self
        {
            Self { alas, tinggi, sisi }
        }

        pub fn luas (&self) -> f64
        {
            (1.00/2.00) * self.alas * self.tinggi
        }

        pub fn keliling (&self) -> f64
        {
            self.sisi[0] + self.sisi[1] + self.sisi[2]
        }
    }

    pub struct Lingkaran
    {
        r: f64,
    }

    impl Lingkaran
    {
        pub fn new (r: f64) -> Self
        {
            Self { r }
        }

        pub fn luas (&self) -> f64
        {
            std::f64::consts::PI * self.r.powf(2.00)
        }

        pub fn keliling (&self) -> f64
        {
            2.00 * std::f64::consts::PI * self.r
        }
    }
}

pub mod bangun_ruang 
{
    pub struct Kubus 
    {
        sisi: f64,
    }

    impl Kubus 
    {
        pub fn new (sisi: f64) -> Self 
        {
            Self { sisi }
        }

        pub fn volume (&self) -> f64 
        {
            self.sisi.powf(3.00)
        }

        pub fn luas_permukaan (&self) -> f64
        {
            6.00 * self.sisi.powf(2.00)
        }

        pub fn diagonal_bidang (&self) -> f64
        {
            self.sisi * 2.00_f64.sqrt()
        }

        pub fn diagonal_ruang (&self) -> f64
        {
            self.sisi * 3.00_f64.sqrt()
        }

        pub fn luas_bidang_diagonal (&self) -> f64
        {
            self.sisi.powf(2.00) * 2.00_f64.sqrt()
        }

        pub fn keliling (&self) -> f64
        {
            self.sisi * 12.00
        }
    }
    
    pub struct Balok 
    {
        panjang: f64, lebar: f64, tinggi: f64,
    }

    impl Balok
    {
        pub fn new (panjang: f64, lebar: f64, tinggi: f64) -> Self
        {
            Self { panjang, lebar, tinggi }
        }

        pub fn volume (&self) -> f64
        {
            self.panjang * self.lebar * self.tinggi
        }

        pub fn keliling (&self) -> f64
        {
            4.00 * (self.panjang + self.lebar + self.tinggi)
        }

        pub fn luas_permukaan (&self) -> f64
        {
            2.00 * (
                (self.panjang * self.lebar) +
                (self.panjang * self.tinggi) +
                (self.lebar * self.tinggi)
            )
        }

        pub fn luas_sis (&self)
        {
            println!("Luas sisi depan: {}\nLuas sisi belakang: {}\nLuas sisi samping: {}",
                self.panjang * self.lebar, self.panjang * self.tinggi, self.lebar * self.tinggi);
        }

        pub fn diagonal_bidang (&self) -> f64
        {
            (self.panjang.powf(2.00) + self.lebar.powf(2.00)).sqrt()
        }

        pub fn diagonal_ruang (&self) -> f64
        {
            (self.panjang.powf(2.00) + self.lebar.powf(2.00) + self.tinggi.powf(2.00)).sqrt()
        }
    }

    pub struct Bola
    {
        r: f64,
    }

    impl Bola
    {
        pub fn new (r: f64) -> Self
        {
            Self { r }
        }

        pub fn luas_permukaan (&self) -> f64
        {
            4.00 * std::f64::consts::PI * self.r.powf(2.00)
        }

        pub fn volume (&self) -> f64
        {
            (4.00 / 3.00) * std::f64::consts::PI * self.r.powf(3.00)
        }

        pub fn setengah_volume (&self) -> f64
        {
            (2.00 / 3.00) * std::f64::consts::PI * self.r.powf(3.00)
        }

        pub fn keliling (&self) -> f64
        {
            2.00 * std::f64::consts::PI * self.r
        }
    }

    pub struct Tabung
    {
        r: f64, tinggi: f64,
    }

    impl Tabung
    {
        pub fn new (r: f64, tinggi: f64) -> Self
        {
            Self { r, tinggi }
        }

        pub fn volume (&self) -> f64
        {
            std::f64::consts::PI * self.r.powf(2.00) * self.tinggi
        }
        
        pub fn luas_alas (&self) -> f64
        {
            2.00 * std::f64::consts::PI * self.r * (self.r + self.tinggi)
        }

        pub fn keliling_alas (&self) -> f64
        {
            2.00 * std::f64::consts::PI * self.r
        }
    }
}
