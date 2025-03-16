mod operasi;
use operasi::aritmetika::*;

fn main(){
    let hasil1 = pangkat(2, 10);
    let hasil2 = pangkat_optim(2, 15);
    let hasil3 = pangkat_desimal(2.3, 3.3);

    println!("Pangkat biasa: {}", hasil1);
    println!("Pangkat cepat: {}", hasil2);
    println!("Pangkat desimal: {}", hasil3);
}
