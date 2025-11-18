use sha2::{Digest,Sha256};
use chrono::prelude::*;

//--------------//
// BLOCK STRUCT (Creating the block structure)
//-------------//
#[derive(Debug)]
struct Block {
    index:u32,
    data:String,
    timestamp:i64,
    previous_hash:String,
    hash:String,
    nonce:u64,
    effort_used:u64
}

// =========================
// IMPLEMENTATION OF BLOCK (Adding methods of operations to the block)
// =========================
impl Block {
    //First method, Create a new block and immediately mine it.
    fn new(index:u32,data:String,previous_hash:String)-> Self {
        let block = Block{
            index,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            effort_used: 0,
            timestamp: chrono::Utc::now().timestamp(),
        };
        block
    }

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


    fn mine_block(&mut self,difficulty: usize){
        //A valid hash must start with `difficulty` number of zeros
        let target:String = "0".repeat(difficulty); // `0000`
       while !self.hash.starts_with(&target){
        self.effort_used += 1;
        self.nonce += 1;
        self.hash = self.calculate_hash();
       }
       println!("⛏️ Block mined: {}", self.hash);
    }
    
}

//--------------//
// BLOCKCHAIN STRUCT //

struct Blockchain {
    chain:Vec<Block>,
    difficulty:usize,
}


//-------------//
// Implementation of the blockchain (Adding methods of operations to the blockchain);

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty: 4, // number of zeros required
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
    fn latest_block(&self){
        self.chain.last().unwrap();
    }

    // Add a new block
    fn add_block(&mut self, data: String) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let index = self.chain.len() as u32;

        let mut new_block = Block::new(index, data,previous_hash);
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    //Check if a block is valid
    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let prev = &self.chain[i - 1];//get the last block in the blockchain
            let current = &self.chain[i]; //get the current block being accessed by the loop;

            //Recalculate the has and compare
            if current.hash != current.calculate_hash(){
                return false;
            }
            //Check if the block correctly references its parent
            if current.previous_hash != prev.hash{
                return false;
            }
        }
        true
    }
}
// =========================
// MAIN PROGRAM
// =========================
fn main() {
    let mut blockchain = Blockchain::new(); // difficulty = 4
    blockchain.add_block("User A pays User B 5 coins".into());
    blockchain.add_block("User C pays User D 2 coins".into());

    // Print the blockchain
    for block in blockchain.chain {
        println!("=========================================");
        println!("Block #{}", block.index);
        println!("Data: {}", block.data);
        println!("Hash: {}", block.hash);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Nonce: {}", block.nonce);
        println!("Effort Used (Attempts): {}", block.effort_used); // <--- effort shown
    }
}
