use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use crate::models::Chain;

pub fn load_chain() -> Chain {
    let path = Path::new("chain.json");
    if path.exists() {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    } else {
        Chain::default()
    }
}

pub fn save_chain(chain: &Chain) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("chain.json")
        .unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, chain).unwrap();
}