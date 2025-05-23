use crate::models::{Chain, Income};
use std::io::{self, Write};
use chrono::Utc;

pub fn add_income(chain: &mut Chain) {
    let mut amount = String::new();
    let mut source = String::new();
    let mut description = String::new();

    print!("Tutar (₺): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut amount).unwrap();

    print!("Kaynak (sponsor): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut source).unwrap();

    print!("Açıklama: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut description).unwrap();

    let income = Income {
        amount: amount.trim().parse().unwrap_or(0.0),
        source: source.trim().to_string(),
        description: description.trim().to_string(),
        timestamp: Utc::now().to_rfc3339(),
    };

    chain.income.push(income);
    println!("✅ Gelir eklendi.");
}