use crate::models::Chain;

pub fn show_impact_score(chain: &Chain) {
    let total_power: u32 = chain.members.iter().map(|m| m.vote_power).sum();
    let frozen_count = chain.members.iter().filter(|m| m.is_frozen).count();
    let delegated_count = chain.members.iter().filter(|m| m.delegated_to.is_some()).count();
    let total_members = chain.members.len();

    println!("\n=== Etki Skoru Raporu ===");
    println!("Toplam Üye: {}", total_members);
    println!("Toplam Oy Gücü (token): {}", total_power);
    println!("Engellenen Kullanıcılar: {}", frozen_count);
    println!("Delege Edilmiş Oylar: {}", delegated_count);
    println!("Toplam Oy: {}", chain.votes.len());
}

pub fn show_members(chain: &Chain) {
    println!("\n=== Üye Listesi ===");
    for m in &chain.members {
        println!(
            "- {} | Kulüp: {} | Oy Gücü: {} | Delege: {:?} | Engel: {}",
            m.name, m.club, m.vote_power, m.delegated_to, m.is_frozen
        );
    }
}

pub fn show_votes(chain: &Chain) {
    println!("\n=== Oy Kayıtları ===");
    for v in &chain.votes {
        println!("- {} → {} ({})", v.voter, v.option, v.topic);
    }
}