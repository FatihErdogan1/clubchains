use crate::models::{Chain, Expense};
use std::io::{self, Write};
use chrono::Utc;

pub fn add_expense(chain: &mut Chain) {
    let mut amount = String::new();
    let mut purpose = String::new();
    let mut invoice_no = String::new();

    print!("Tutar (₺): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut amount).unwrap();

    print!("Açıklama: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut purpose).unwrap();

    print!("Fatura No: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut invoice_no).unwrap();

    let expense = Expense {
        amount: amount.trim().parse().unwrap_or(0.0),
        purpose: purpose.trim().to_string(),
        invoice_no: invoice_no.trim().to_string(),
        timestamp: Utc::now().to_rfc3339(),
    };

    chain.expenses.push(expense);
    println!("✅ Gider eklendi.");
}