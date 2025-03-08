pub mod operasi {
    pub mod aritmetika;
}

pub mod sistem {
    pub mod kelipatan;
}

use crate::operasi::aritmetika::*;
use crate::sistem::kelipatan::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = tambah(6.5, 6.0);
        let angka = [2,3,4,5];
        let super_result = super_tambah(&angka);
        let hasil_pangkat = pangkat(2, 3);
        let hasil_kpk = kpk(12, 18);
        let hasil_fpb = fpb(12, 18);
    }
}
