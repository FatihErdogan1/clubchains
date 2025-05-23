use crate::models::Chain;
use std::io::{self, Write};

pub fn burn_token(chain: &mut Chain) {
    let mut name = String::new();
    let mut amount = String::new();

    print!("Token eksiltilecek üye adı: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("Eksiltilecek token miktarı: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut amount).unwrap();

    let amount: u32 = amount.trim().parse().unwrap_or(0);
    if let Some(member) = chain.members.iter_mut().find(|m| m.name == name.trim()) {
        if member.vote_power >= amount {
            member.vote_power -= amount;
            println!("🔥 {} kişisinden {} token silindi.", member.name, amount);
        } else {
            println!("⚠️ {} kişisinin yeterli token'ı yok.", member.name);
        }
    } else {
        println!("❌ Kullanıcı bulunamadı.");
    }
}