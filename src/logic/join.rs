use crate::models::{Member, Chain};
use std::io::{self, Write};

pub fn add_member(chain: &mut Chain) {
    let mut name = String::new();
    let mut club = String::new();

    print!("İsim: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("Kulüp: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut club).unwrap();

    let member = Member {
        name: name.trim().to_string(),
        club: club.trim().to_string(),
        vote_power: 1,              // başlangıçta 1 oy gücü
        delegated_to: None,
        is_frozen: false,
    };

    chain.members.push(member);
    println!("✅ Üyelik kaydı başarılı.");
}