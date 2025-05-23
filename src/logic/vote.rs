use crate::models::{Chain, Vote};
use std::io::{self, Write};

pub fn cast_vote(chain: &mut Chain) {
    let mut voter = String::new();
    let mut topic = String::new();
    let mut option = String::new();

    print!("Oy veren kişi (isim): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut voter).unwrap();
    let voter = voter.trim();

    print!("Oylama konusu: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut topic).unwrap();
    let topic = topic.trim();

    print!("Tercih (seçenek): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option).unwrap();
    let option = option.trim();

    if let Some(member) = chain.members.iter_mut().find(|m| m.name == voter) {
        if member.is_frozen {
            println!("🚫 Bu kullanıcı engellenmiş ve oy kullanamaz.");
            return;
        }

        if member.vote_power == 0 {
            println!("⚠️ Oy kullanacak yeterli token (vote_power) yok.");
            return;
        }

        // Oy kaydı
        let vote = Vote {
            topic: topic.to_string(),
            voter: voter.to_string(),
            option: option.to_string(),
        };

        chain.votes.push(vote);
        println!("✅ {} konusu için oy kaydedildi.", topic);
    } else {
        println!("❌ Böyle bir kullanıcı bulunamadı.");
    }
}