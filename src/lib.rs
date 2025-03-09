pub mod operasi {
    pub mod aritmetika;
    pub mod kombinatorika;
}

pub mod sistem {
    pub mod kelipatan;
    pub mod bilangan;
    pub mod basis;
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // * ============================================================ *
        //  Kalau memang dibutuhkan, juga bisa buat ngasih contoh sintaks.
        // * ============================================================ *
        
        use crate::sistem::kelipatan; 
        use crate::sistem::bilangan;

        let x = 6.7487; let y: i64 = 8;
        let z: i64 = bilangan::bulat(x) as i64;
        bilangan::genap("cek", kelipatan::kpk(z, y));
        bilangan::ganjil("rubah", kelipatan::fpb(z, y));

    }
}
