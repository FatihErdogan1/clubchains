use crate::models::{Chain, Event};
use std::io::{self, Write};
use chrono::Utc;

pub fn create_event(chain: &mut Chain) {
    let mut name = String::new();
    let mut organizer = String::new();

    print!("Etkinlik adı: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("Organizatör: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut organizer).unwrap();

    let event = Event {
        name: name.trim().to_string(),
        organizer: organizer.trim().to_string(),
        date: Utc::now().to_rfc3339(),
    };

    chain.events.push(event);
    println!("✅ Etkinlik oluşturuldu.");
}
pub fn show_events(chain: &Chain) {
    println!("\n=== Etkinlik Listesi ===");
    for event in &chain.events {
        println!(
            "- {} | Düzenleyen: {} | Tarih: {}",
            event.name, event.organizer, event.date
        );
    }
}