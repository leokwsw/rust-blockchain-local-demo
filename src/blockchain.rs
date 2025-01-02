use serde::{Deserialize, Serialize};
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
        Self {
            blocks: vec![Block::new("Genesis Block".into(), "0".into())],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = Block::new(data, previous_hash);
        self.blocks.push(new_block);
    }
}

// fn main() {
//     let block = Block {
//         id: Uuid::new_v4(),
//         timestamp: 1234567890,
//         data: "Test Data".into(),
//         previous_hash: "0000".into(),
//         hash: "abcd1234".into(),
//     };
//
//     let serialized = serde_json::to_string(&block).unwrap();
//     println!("Serialized Block: {}", serialized);
//
//     let deserialized: Block = serde_json::from_str(&serialized).unwrap();
//     println!("Deserialized Block: {:?}", deserialized);
// }
