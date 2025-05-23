use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Member {
    pub name: String,
    pub club: String,
    pub vote_power: u32,
    pub delegated_to: Option<String>,
    pub is_frozen: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vote {
    pub topic: String,
    pub voter: String,
    pub option: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub name: String,
    pub organizer: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Certificate {
    pub event: String,
    pub recipient: String,
    pub token_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Income {
    pub amount: f64,
    pub source: String,
    pub description: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expense {
    pub amount: f64,
    pub purpose: String,
    pub invoice_no: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Chain {
    pub members: Vec<Member>,
    pub votes: Vec<Vote>,
    pub events: Vec<Event>,
    pub certificates: Vec<Certificate>,
    pub income: Vec<Income>,
    pub expenses: Vec<Expense>,
}