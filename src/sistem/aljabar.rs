pub struct Aljabar;

impl Aljabar
{
    // 
    // Sistem Persamaan Linear Satu Variable
    // x = -b / a
    //
    pub fn splsv(a: f64, b: f64) -> Option<f64> 
    {
        if a == 0.0 { return None; }
        Some(-b / a)
    }
    
    // 
    // Sistem Persamaan Linear Dua Variable
    // a1x + b1y = c1
    // a2x + b2y = c2
    //
    pub fn spldv (
    a1: f64, b1: f64, c1: f64,
    a2: f64, b2: f64, c2: f64 ) -> Option<(f64, f64)> {
    
    let det = a1 * b2 - a2 * b1;
    if det == 0.0 { return None; }

    // Hitung nilai y
    let y = (a1 * c2 - a2 * c1) / det;
    // Hitung nilai x
    let x = (c1 - b1 * y) / a1;

    Some((x, y))
}
    
    // 
    // Sistem Persamaan Linear Tiga Variable
    // a1x - b1y + c1z = d1
    // a2x + b2y - c2z = d2
    // a3x - b3y + c3z = d3
    //
    pub fn spltv(
        a1: f64, b1: f64, c1: f64, d1: f64,
        a2: f64, b2: f64, c2: f64, d2: f64,
        a3: f64, b3: f64, c3: f64, d3: f64
    ) -> Option<(f64, f64, f64)> 
    {
        // Determinan utama
        let det = a1 * (b2 * c3 - b3 * c2)
                - b1 * (a2 * c3 - a3 * c2)
                + c1 * (a2 * b3 - a3 * b2);
        
        if det == 0.0 { return None; }
        
        // Determinan x
        let det_x = d1 * (b2 * c3 - b3 * c2)
                  - b1 * (d2 * c3 - d3 * c2)
                  + c1 * (d2 * b3 - d3 * b2);
        // Determinan y
        let det_y = a1 * (d2 * c3 - d3 * c2)
                  - d1 * (a2 * c3 - a3 * c2)
                  + c1 * (a2 * d3 - a3 * d2);
        // Determinan z
        let det_z = a1 * (b2 * d3 - b3 * d2)
                  - b1 * (a2 * d3 - a3 * d2)
                  + d1 * (a2 * b3 - a3 * b2);
        // Menghitung x, y, dan z
        let x = det_x / det;
        let y = det_y / det;
        let z = det_z / det;
        
        Some((x, y, z))
    }
}
