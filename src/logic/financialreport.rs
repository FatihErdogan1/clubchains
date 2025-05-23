use crate::models::Chain;

pub fn show_finance_report(chain: &Chain) {
    let total_income: f64 = chain.income.iter().map(|i| i.amount).sum();
    let total_expense: f64 = chain.expenses.iter().map(|e| e.amount).sum();
    let balance = total_income - total_expense;

    println!("\n=== Finansal Rapor ===");
    println!("Toplam Gelir: {:.2} ₺", total_income);
    println!("Toplam Gider: {:.2} ₺", total_expense);
    println!("Net Bakiye: {:.2} ₺", balance);
}