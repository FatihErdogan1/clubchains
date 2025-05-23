use crate::models::Chain;
use std::io::{self, Write};

pub fn delegate_vote(chain: &mut Chain) {
    let mut from = String::new();
    let mut to = String::new();

    print!("Oy hakkını devreden kişi: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut from).unwrap();

    print!("Oy hakkını devralacak kişi: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut to).unwrap();

    let from = from.trim();
    let to = to.trim();

    let mut found = false;

    for member in &mut chain.members {
        if member.name == from {
            if member.vote_power == 0 {
                println!("⚠️ {} kullanıcısının devredecek oyu yok.", from);
                return;
            }
            member.delegated_to = Some(to.to_string());
            println!("✅ Oy hakkı {} → {} olarak devredildi.", from, to);
            found = true;
        }
    }

    if !found {
        println!("❌ Devreden kullanıcı bulunamadı.");
    }
}