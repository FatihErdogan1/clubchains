use crate::models::*;
use chrono::Utc;

pub fn load_demo_data(chain: &mut Chain) {
    // Üyeler
    let names = vec!["Ece", "Ahmet", "Zeynep", "Mehmet", "Ayşe"];
    let clubs = vec!["Yapay Zeka", "Robotik", "Girişimcilik"];
    for (i, name) in names.iter().enumerate() {
        chain.members.push(Member {
            name: name.to_string(),
            club: clubs[i % clubs.len()].to_string(),
            vote_power: 2,
            delegated_to: None,
            is_frozen: false,
        });
    }

    // Oylar
    chain.votes.push(Vote {
        topic: "Etkinlik Lokasyonu".to_string(),
        voter: "Ece".to_string(),
        option: "İstanbul".to_string(),
    });

    // Etkinlikler
    chain.events.push(Event {
        name: "Rust Hackathon".to_string(),
        organizer: "Yapay Zeka".to_string(),
        date: Utc::now().to_rfc3339(),
    });

    // Sertifikalar
    chain.certificates.push(Certificate {
        event: "Rust Hackathon".to_string(),
        recipient: "Ahmet".to_string(),
        token_id: "CERT-123456".to_string(),
    });

    // Gelirler
    chain.income.push(Income {
        amount: 5000.0,
        source: "Stellar".to_string(),
        description: "Sponsor destek".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    });

    // Giderler
    chain.expenses.push(Expense {
        amount: 1200.0,
        purpose: "Pizza ve içecek".to_string(),
        invoice_no: "INV-001".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    });

    println!("🎉 Demo verileri başarıyla yüklendi!");
}