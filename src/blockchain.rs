use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: Uuid,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(data: String, previous_hash: String) -> Self {
        let id = Uuid::new_v4();
        let timestamp = chrono::Utc::now().timestamp_millis() as u64;
        let hash = format!(
            "{:x}",
            md5::compute(format!(
                "{:?}{:?}{:?}{:?}",
                id, timestamp, &data, &previous_hash
            ))
        );
        Self {
            id,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        if let Ok(blocks) = Self::load_from_file("blockchain.json") {
            Blockchain { blocks }
        } else {
            Blockchain {
                blocks: vec![Block::new("Genesis Block".into(), "0".into())],
            }
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = Block::new(data, previous_hash);
        self.blocks.push(new_block);
        self.save_to_file("blockchain.json").unwrap();
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        let json = serde_json::to_string_pretty(&self.blocks)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> std::io::Result<Vec<Block>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let blocks = serde_json::from_reader(reader)?;
        Ok(blocks)
    }
}
