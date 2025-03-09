pub fn konversi_basis(mut num: u64, base: u32) -> String 
{
    if base < 2 || base > 36 {
        panic!("Basis harus antara 2 hingga 36");
    }
    if num == 0 {
        return "0".to_string();
    }
    let mut hasil = String::new();
    while num > 0 
    {
        let rem = (num % base as u64) as u32;
        let digit = if rem < 10 {
            std::char::from_digit(rem, base).unwrap()
        } else {
            std::char::from_u32('A' as u32 + rem - 10).unwrap()
        };
        hasil.push(digit);
        num /= base as u64;
    }
    hasil.chars().rev().collect()
}

pub fn parse_number(num_str: &str, from_base: u32) -> u64 
{
    if from_base < 2 || from_base > 36 {
        panic!("Basis harus antara 2 hingga 36");
    }
    let mut result: u64 = 0;
    for c in num_str.chars() 
    {
        let digit = c
            .to_digit(from_base)
            .expect("Digit tidak valid untuk basis yang diberikan");
        result = result * (from_base as u64) + digit as u64;
    }
    result
}

/// === Konversi khusus antar basis ===

pub fn desimal_ke_biner(num: u64) -> String { konversi_basis(num, 2) }
pub fn desimal_ke_oktal(num: u64) -> String { konversi_basis(num, 8) }
pub fn desimal_ke_hexadesimal(num: u64) -> String { konversi_basis(num, 16) }

pub fn biner_ke_desimal(num_str: &str) -> u64 { parse_number(num_str, 2) }
pub fn biner_ke_oktal(num_str: &str) -> String 
{
    let dec = biner_ke_desimal(num_str);
    desimal_ke_oktal(dec)
}
pub fn biner_ke_hexadesimal(num_str: &str) -> String 
{
    let dec = biner_ke_desimal(num_str);
    desimal_ke_hexadesimal(dec)
}

pub fn hexadesimal_ke_desimal(num_str: &str) -> u64 { parse_number(num_str, 16) }
pub fn hexadesimal_ke_biner(num_str: &str) -> String 
{
    let dec = hexadesimal_ke_desimal(num_str);
    desimal_ke_biner(dec)
}
pub fn hexadesimal_ke_oktal(num_str: &str) -> String 
{
    let dec = hexadesimal_ke_desimal(num_str);
    desimal_ke_oktal(dec)
}

pub fn oktal_ke_desimal(num_str: &str) -> u64 { parse_number(num_str, 8) }
pub fn oktal_ke_biner(num_str: &str) -> String 
{
    let dec = oktal_ke_desimal(num_str);
    desimal_ke_biner(dec)
}
pub fn oktal_ke_hexadesimal(num_str: &str) -> String 
{
    let dec = oktal_ke_desimal(num_str);
    desimal_ke_hexadesimal(dec)
}

