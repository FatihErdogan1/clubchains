use std::process::Command;
use std::io::{self, Write};

pub fn mint_token() {
    let contract_id = "CA7D5XVD3JAYV4I2Q47Q7ZJY4POXMF2XISUYIOESWOFONQQ3ZUZ27VNB";

    let mut user_address = String::new();
    let mut amount = String::new();

    print!("Kullanıcı Stellar adresi: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_address).unwrap();

    print!("Verilecek token miktarı: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut amount).unwrap();

    let output = Command::new("stellar")
        .args(&[
            "contract", "invoke",
            "--id", contract_id,
            "--source", "fatih276",
            "--network", "testnet",
            "--", "mint",
            "--to", user_address.trim(),
            "--amount", amount.trim(),
        ])
        .output()
        .expect("❌ Soroban mint işlemi başarısız oldu.");

    println!("🪙 Zincir Çıktısı: {}", String::from_utf8_lossy(&output.stdout));
}
