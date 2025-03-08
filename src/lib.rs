pub mod operasi {
    pub mod aritmetika;
}

use crate::operasi::aritmetika::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = tambah(6.5, 6.0);
        let angka = [2,3,4,5];
        let super_result = super_tambah(&angka);
        let hasil_pangkat = pangkat(2, 3);
    }
}
