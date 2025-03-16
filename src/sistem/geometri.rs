pub mod dua_dimensi 
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
            self.sisi * 3.00 
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
}

pub mod tiga_dimensi 
{
    pub struct Kubus 
    {
        sisi: f64, panjang: f64,
    }

    impl Kubus 
    {
        pub fn new (sisi: f64, panjang: f64) -> Self 
        {
            Self { sisi, panjang }
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
            self.sisi * self.panjang
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
}
