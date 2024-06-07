use std::io::{self, Write};

struct PersegiP {
    lebar: f64,
    panjang: f64,
}

impl PersegiP {
    fn new(lebar: f64, panjang: f64) -> Result<Self, &'static str> {
        if lebar <= 0.0 || panjang <= 0.0 {
            return Err("Lebar dan panjang harus angka positif.");
        }
        Ok(PersegiP { lebar, panjang })
    }
    fn hitung_luas(&self) -> f64 {
        self.lebar * self.panjang
    }
    fn hitung_keliling(&self) -> f64 {
        2.0 * (self.lebar + self.panjang)
    }
}

fn main() {
    let mut lebar_str = String::new();
    let mut panjang_str = String::new();

    println!("Masukkan lebar persegi panjang: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lebar_str).unwrap();

    println!("Masukkan panjang persegi panjang: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut panjang_str).unwrap();

    let lebar: f64 = match lebar_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Gagal membaca input.");
            return;
        }
    };

    let panjang: f64 = match panjang_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Gagal membaca input.");
            return;
        }
    };

    let persegi_panjang = match PersegiP::new(lebar, panjang) {
        Ok(persegi) => persegi,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let luas = persegi_panjang.hitung_luas();
    let keliling = persegi_panjang.hitung_keliling();

    println!("Luas persegi panjang: {} satuan persegi", luas);
    println!("Keliling persegi panjang: {} satuan", keliling);
}
