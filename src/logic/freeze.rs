use crate::models::Chain;
use std::io::{self, Write};

pub fn freeze_member(chain: &mut Chain) {
    let mut name = String::new();
    print!("Engellenecek kullanıcı adı: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    if let Some(member) = chain.members.iter_mut().find(|m| m.name == name) {
        member.is_frozen = true;
        println!("🚫 {} kullanıcısı engellendi.", name);
    } else {
        println!("❌ Kullanıcı bulunamadı.");
    }
}