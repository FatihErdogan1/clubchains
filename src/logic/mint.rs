use crate::models::Chain;
use std::io::{self, Write};

pub fn mint_token(chain: &mut Chain) {
    let mut name = String::new();
    let mut amount = String::new();

    print!("Token verilecek üye adı: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("Verilecek token miktarı: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut amount).unwrap();

    let amount: u32 = amount.trim().parse().unwrap_or(0);
    if let Some(member) = chain.members.iter_mut().find(|m| m.name == name.trim()) {
        member.vote_power += amount;
        println!("✅ {} kişisine {} token eklendi.", member.name, amount);
    } else {
        println!("❌ Kullanıcı bulunamadı.");
    }
}