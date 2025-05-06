pub struct SistemPersamaan;

impl SistemPersamaan
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
    pub fn spldv(
        a1: f64, b1: f64, c1: f64,
        a2: f64, b2: f64, c2: f64
    ) -> Option<(f64, f64)>
    {
        let new_b1; let new_b2;
        let new_c1; let new_c2;

        if a1 == a2 {                 // Kalau koefisien x pertama
            new_b1 = b1; new_b2 = b2; // sama dengan koefisien x kedua,
            new_c1 = c1; new_c2 = c2; // maka kedua persamaan tidak ada yang berubah.
        } else {
            new_b1 = b1 * a2; new_b2 = b2 * a1; // Kalau tidak, maka persamaan pertama
            new_c1 = c1 * a2; new_c2 = c2 * a1; // dikalikan dengan a1 dan persamaan kedua
                                            // dikalikan denga a2 (koefisien 1 dan 2 x).
        }

        let b3; let c3;

        if a1 < 0.00 || a2 < 0.00 { // Jika a1 atau a2 bernilai negatif,
            b3 = new_b1 + new_b2;   // Maka kedua persamaan akan dijumlah.
            c3 = new_c1 + new_c2;
        } else {                    // Sebaliknya, jika salah satu koefisien
            b3 = new_b1 - new_b2;   // bernilai positif, maka kedua persamaan
            c3 = new_c1 - new_c2;   // akan dikurangi.
        }

        let y = c3 / b3;
        let x = (c2 - (b2 * y)) / a2;

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

    pub fn splsv_proses(a: f64, b: f64) -> (Option<f64>, String) {
        let mut steps = format!("Persamaan: {:.2}x + {:.2} = 0\n", a, b);

        if a == 0.0 {
            steps.push_str("Koefisien x (a) = 0 → Tidak ada solusi.\n");
            return (None, steps);
        }

        let x = -b / a;
        steps.push_str(&format!(
            "Pindahkan b ke kanan:\n  {:.2}x = {:.2}\nBagi dengan a:\n  x = {:.2}\n",
            a, -b, x
        ));

        (Some(x), steps)
    }

    pub fn spldv_proses(
        a1: f64, b1: f64, c1: f64,
        a2: f64, b2: f64, c2: f64
    ) -> (Option<(f64, f64)>, String) {
        let mut steps = String::new();

        steps.push_str(&format!(
            "Persamaan:\n  a1x + b1y = c1  →  {:.2}x + {:.2}y = {:.2}\n  a2x + b2y = c2  →  {:.2}x + {:.2}y = {:.2}\n\n",
            a1, b1, c1, a2, b2, c2
        ));

        let (new_b1, new_b2, new_c1, new_c2) = if a1 == a2 {
            steps.push_str("Karena a1 == a2, persamaan tidak diubah.\n\n");
            (b1, b2, c1, c2)
        } else {
            steps.push_str(&format!(
                "Mengalikan P1 dengan {:.2}, dan P2 dengan {:.2}:\n",
                a2, a1
            ));
            (
                b1 * a2,
                b2 * a1,
                c1 * a2,
                c2 * a1
            )
        };

        let (b3, c3) = if a1 < 0.0 || a2 < 0.0 {
            steps.push_str("a1 atau a2 negatif → Jumlahkan kedua persamaan:\n");
            (new_b1 + new_b2, new_c1 + new_c2)
        } else {
            steps.push_str("a1 dan a2 positif → Kurangkan kedua persamaan:\n");
            (new_b1 - new_b2, new_c1 - new_c2)
        };

        steps.push_str(&format!(
            "Hasil eliminasi x:\n  {:.2}y = {:.2}\n",
            b3, c3
        ));

        if b3 == 0.0 {
            return (None, steps + "\nTidak dapat menyelesaikan karena pembagi 0.");
        }

        let y = c3 / b3;
        steps.push_str(&format!("  y = {:.2}\n", y));

        let x = (c2 - (b2 * y)) / a2;
        steps.push_str(&format!("\nSubstitusi y ke P2:\n  x = ({:.2} - {:.2} × {:.2}) / {:.2} = {:.2}\n", c2, b2, y, a2, x));

        (Some((x, y)), steps)
    }

}
