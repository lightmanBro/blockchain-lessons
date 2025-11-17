// ---------- IMPORTS ----------
use sha2::{Sha256, Digest};
use chrono::prelude::*;

// This struct represents a single block in the blockchain.
#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    // Create a new block (not yet mined)
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        Block {
            index,
            timestamp: Utc::now().timestamp(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    // Convert block fields to one string for hashing
    fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.data,
            self.previous_hash,
            self.nonce
        );

        // Compute SHA-256 hash
        let mut hasher = Sha256::new();
        hasher.update(block_data.as_bytes());
        let result = hasher.finalize();

        format!("{:x}", result)
    }

    // Mining: find a hash with difficulty leading zeros
    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty); // e.g., difficulty=3 → "000"

        // Loop until the hash starts with enough zeros
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        println!("⛏️ Block mined: {}", self.hash);
    }
}

// ---------- BLOCKCHAIN ----------
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize, // mining difficulty
}

impl Blockchain {
    // Create a new blockchain with a genesis block
    fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty: 3, // number of zeros required
        };

        blockchain.create_genesis_block();
        blockchain
    }

    // First block in blockchain
    fn create_genesis_block(&mut self) {
        let mut genesis = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        genesis.hash = genesis.calculate_hash();
        self.chain.push(genesis);
    }

    // Get the last block
    fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    // Add a new block to the chain
    fn add_block(&mut self, data: String) {
        let previous_hash = self.latest_block().hash.clone();
        let index = self.chain.len() as u64;

        let mut block = Block::new(index, data, previous_hash);
        block.mine_block(self.difficulty);
        self.chain.push(block);
    }

    // Validate entire chain (check for tampering)
    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let prev = &self.chain[i - 1];
            let curr = &self.chain[i];

            // Recalculate the hash and compare
            if curr.hash != curr.calculate_hash() {
                return false;
            }

            // Check if the block correctly references its parent
            if curr.previous_hash != prev.hash {
                return false;
            }
        }
        true
    }
}

// ---------- MAIN ----------
fn main() {
    let mut blockchain = Blockchain::new();

    println!("⛓️ Mining block 1...");
    blockchain.add_block("Alice paid Bob 5 BTC".to_string());

    println!("⛓️ Mining block 2...");
    blockchain.add_block("Bob paid Charlie 2 BTC".to_string());

    println!("Blockchain valid? {}", blockchain.is_valid());
}
