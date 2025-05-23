use crate::models::{Chain, Certificate};
use std::io::{self, Write};
use chrono::Utc;

pub fn issue_certificate(chain: &mut Chain) {
    let mut event = String::new();
    let mut recipient = String::new();

    print!("Etkinlik adı: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut event).unwrap();

    print!("Katılımcı adı: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut recipient).unwrap();

    let cert = Certificate {
        event: event.trim().to_string(),
        recipient: recipient.trim().to_string(),
        token_id: format!("CERT-{}", Utc::now().timestamp()),
    };

    chain.certificates.push(cert);
    println!("🎓 Sertifika verildi!");
}
pub fn show_certificates(chain: &Chain) {
    println!("\n=== Sertifika Listesi ===");
    for cert in &chain.certificates {
        println!(
            "- {} etkinliği için {} adlı kişiye verildi (ID: {})",
            cert.event, cert.recipient, cert.token_id
        );
    }
}