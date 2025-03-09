use matematika_rs::operasi::aritmetika;
use matematika_rs::operasi::kombinatorika;
use matematika_rs::sistem::*;

fn main() {
    println!("{}", aritmetika::tambah(6, 7));
    println!("{}", kombinatorika::kombinasi(6, 2));
    println!("{}", bilangan::bulat(aritmetika::pangkat(6.578, 2)));
    let x = 100; let y = 5;
    let z = [5, 6, 8, 125];
    let hasil = aritmetika::tambah(
                aritmetika::tambah(aritmetika::tambah(x, y), aritmetika::kali(x, y)), 
                aritmetika::super_kurang(&z));    
    println!("{}", hasil);
    let x = 6.7487; let y: i64 = 8;
    let z: i64 = bilangan::bulat(x) as i64;
    bilangan::genap("cek", kelipatan::kpk(z, y));
    bilangan::ganjil("rubah", kelipatan::fpb(z, y));
    let x: u64 = 680; let y: u64 = 87;
    println!("{}\n{}\n{}\n{}",
        basis::konversi_basis(aritmetika::tambah(x ,y), 2),
        basis::desimal_ke_biner(aritmetika::kali(x, y)),
        basis::biner_ke_hexadesimal(
           &basis::desimal_ke_biner(aritmetika::kurang(x, y)) 
        ),
        basis::hexadesimal_ke_oktal(
            &basis::desimal_ke_hexadesimal(aritmetika::bagi(x, y))
        )
    );
}
