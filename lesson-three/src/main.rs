/*
What you’ll learn in this lesson

How to create Ed25519 keypairs in Rust.

How to sign a transaction and serialize the canonical bytes to sign.

How to verify signatures when blocks are added.

How tampering breaks verification.

How transaction signatures fit with effort (PoW) — i.e., you don’t mine unsigned/invalid txs.
*/

//
// Simple blockchain with:
// - Transaction signing using Ed25519
// - Mining (proof-of-work) with effort counting (attempts)
// - Verification of transaction signatures before adding a block
//
// This is for learning — not production.

use sha2::{Digest,Sha256};
use chrono::Utc;
use serde::{Serialize,Deserialize};
use ed25519_dalek::{Keypair, Signer, Verifier, PublicKey, Signature};
use rand::rngs::OsRng;
use hex;

#[derive[Debug,Serialize,Deserialize,Clone]]
struct Transaction {
    from: String,                 // sender address (pubkey hex)
    to: String,                   // receiver address (pubkey hex)
    amount: u64,
    signature: Option<Vec<u8>>,   // signature bytes (None until signed)
    pub_key: Option<Vec<u8>>,     // sender public key bytes (for verification)
}

impl Transaction {
    fn new(from:String,to:String,amount:u64) -> Self {
        Self {from,to,amount,signature:None,pub_key:None};
    }

    ///Canonical bytes representation used for signing/verification
    /// We exclude `signature` and `pub_key` fields from the signed documents
    fn bytes_for_signing(&self) -> Vec<u8>{
        //We use json canonical string for simplicity (could use bincode or custom canonicalization).
        //The order of key matter for deterministic signature input.
        let data = serde_json::json!({
            "from":self.from,
            "to":self.to,
            "amount":self.amount
        });

        //Hash the string to keep fixed-size input (Not required for Ed25519, but Ok)
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hasher.finalize().to_vec();
    }

    ///Sign the transaction using the sender's keypair.
    /// Sets signature and public key fields.
    fn sign(&mut self,keypair:&keypair){
        let msg = self.bytes_for_signing();
        let sig: Signature = keypair.sign(&msg);
        self.signature = Some(sig.to_bytes().to_vec());
        self.pub_key = SOme(keypair.public.to_bytes().to_vec());
    }
}
fn main() {
    println!("Hello, world!");
}
