use crate::logic;
use crate::storage::{load_chain, save_chain};
use crate::demo;
use std::io::{self, Write};

pub fn run() {
    let mut chain = load_chain();

    loop {
        println!("\n=== ClubChain Ana Menü ===");
        println!("1. Üye Kaydı");
        println!("2. Oy Kullan");
        println!("3. Oy Hakkı Devret (Delegasyon)");
        println!("4. Kullanıcıyı Engelle");
        println!("5. Etki Skoru Raporu");
        println!("6. Üye Listesi");
        println!("7. Oy Kayıtları");
        println!("8. Token Ver ");
        println!("9. Token Eksilt ");
        println!("10. Etkinlik Oluştur");
        println!("11. Gelir Ekle");
        println!("12. Gider Ekle");
        println!("13. Sertifika Ver");
        println!("14. Finansal Rapor");
        println!("15. Demo Verileri Yükle");
        println!("16. Etkinlik Listesi");
        println!("17. Sertifika Listesi");
        println!("0. Çıkış");
        print!("Seçiminiz: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => logic::join::add_member(&mut chain),
            "2" => logic::vote::cast_vote(&mut chain),
            "3" => logic::delegate::delegate_vote(&mut chain),
            "4" => logic::freeze::freeze_member(&mut chain),
            "5" => logic::report::show_impact_score(&chain),
            "6" => logic::report::show_members(&chain),
            "7" => logic::report::show_votes(&chain),
            "8" => logic::mint::mint_token(),
            "9" => logic::burn::burn_token(),
            "10" => logic::event::create_event(&mut chain),
            "11" => logic::income::add_income(&mut chain),
            "12" => logic::expense::add_expense(&mut chain),
            "13" => logic::certificate::issue_certificate(&mut chain),
            "14" => logic::financialreport::show_finance_report(&chain),
            "15" => demo::load_demo_data(&mut chain),
            "16" => logic::event::show_events(&chain),
            "17" => logic::certificate::show_certificates(&chain),
            "0" => {
                println!("Çıkış yapılıyor...");
                break;
            }
            _ => println!("Geçersiz seçim. Lütfen tekrar deneyin."),
        }

        save_chain(&chain);
    }
}