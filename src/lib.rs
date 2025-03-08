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
    }
}
