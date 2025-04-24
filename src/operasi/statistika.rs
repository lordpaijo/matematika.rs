use std::collections::HashMap;

pub fn mean (angka: &[f64]) -> f64
{
    if angka.is_empty()
    {
        panic!("Tidak bisa menghitung rata-rata dari array kosong.");
    }
    let total: f64 = angka.iter().sum();
    total / angka.len() as f64
}

pub fn median (angka: &mut [f64]) -> f64
{
    if angka.is_empty()
    {
        panic!("Tidak dapat menghitung median dari data kosong.");
    }

    angka.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = angka.len();

    if n % 2 == 1{ angka[n / 2] }
    else { (angka[n / 2 - 1] + angka[n / 2]) / 2.0 }
}

pub fn modus (angka: &[i64]) -> Vec<i64>
{
    if angka.is_empty()
    {
        panic!("Tidak dapat menghitung modus dari data kosong.");
    }

    let mut frekuensi = HashMap::new();
    for &num in angka
    {
        *frekuensi.entry(num).or_insert(0) += 1;
    }

    let max_freq = frekuensi.values().copied().max().unwrap();
    frekuensi
        .into_iter()
        .filter(|&(_, count)| count == max_freq)
        .map(|(num, _)| num)
        .collect()
}
