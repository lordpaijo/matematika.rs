pub mod operasi; 
pub mod sistem;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // * ============================================================ *
        //  Kalau memang dibutuhkan, juga bisa buat ngasih contoh sintaks.
        // * ============================================================ *
        
        use crate::sistem::kelipatan; 
        use crate::sistem::bilangan;
        use crate::sistem::fibonacci;

        let x = 6.7487; let y: i64 = 8;
        let z: i64 = bilangan::bulat(x) as i64;
        bilangan::genap("cek", kelipatan::kpk(z, y));
        bilangan::ganjil("rubah", kelipatan::fpb(z, y));
        fibonacci::Fibonacci::iteratif(10);
    }
}
