mod integral {
    // membuat sebuah fungsi integral dengan metode riemann
    pub fn integral_riemann(){

        //  membuat sebuah variabel delta_x
        let mut delta_x = (batas_atas - batas_bawah) / n;
    
        // menggunakan metode trapezoid
        let mut area_trapezoid = 0.5 * (f_1 + f_2);

        // menggunakan looping
        for i in 1..n{
            let x = a * h;
            sum += f_x
        }

        sum * h
    } 
}

// contoh kasus
    fn main(){
        // input fungsi x^2 dengan batas bawah 0 & batas atas 15, dengan n 1600
        let hasil_integral = integral_riemann(x * x, 0, 15, 1600);
        println!("Hasil Integral : {}", hasil_integral);
    }