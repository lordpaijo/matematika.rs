pub mod operasi {
    pub mod aritmetika;
    pub mod kombinatorika;
}

pub mod sistem {
    pub mod kelipatan;
    pub mod bilangan;
}

use crate::operasi::aritmetika::*;
use crate::operasi::kombinatorika::*;
use crate::sistem::kelipatan::*;
use crate::sistem::bilangan::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _result = tambah(6.5, 6.0);
        let angka = [2,3,4,5];
        let _super_result = super_tambah(&angka);
        let _hasil_pangkat = pangkat(2, 3);
        let _hasil_kpk = kpk(12, 18);
        let _hasil_fpb = fpb(12, 18);
        let _sigma = bulat(4.65);
        let _kena_akar = akar_kuadrat(10.00);
        let _faklah = faktorial(5);
        let dadu = kombinasi(6, 2);
    }
}
