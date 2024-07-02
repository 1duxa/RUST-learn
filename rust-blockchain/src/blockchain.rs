use std::collections::HashMap;

use failure::format_err;
use log::info;

use crate::{
    block::Block,
    errors::Result,
    transaction::Transaction,
    tx::{TXOutput, TXOutputs},
    TARGET_HEXT,
};

#[derive(Debug)]
pub struct Blockchain {
    current_hash: String,
    db: sled::Db,
}
pub struct BcIter<'a> {
    current_hash: String,
    bc: &'a Blockchain,
}

const GENESIS_COINBASE_DATA: &str = "";
impl Blockchain {
    pub fn new() -> Result<Self> {
        info!("open blockchain");

        let db = sled::open("data/blocks")?;
        let hash = db.get("LAST").expect("Create a db block first").unwrap();
        info!("Found block database");

        let lasthash = String::from_utf8(hash.to_vec())?;

        Ok(Self {
            current_hash: lasthash.clone(),
            db,
        })
    }
    pub fn create_blockchain(address: String) -> Result<Self> {
        info!("Creating new blockchain...");

        let db = sled::open("data/blocks")?;
        info!("Creating new database");

        let cbtx = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        let genesis: Block = Block::new_genesis_block(cbtx);
        db.insert(genesis.get_hash(), bincode::serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;

        let bc = Blockchain {
            current_hash: genesis.get_hash(),
            db,
        };
        let _ = bc.db.flush()?;

        Ok(bc)
    }
    pub fn push_block(&mut self, data: Vec<Transaction>) -> Result<()> {
        let lasthash = self.db.get("LAST")?.expect("Failed to get last");
        let new_block = Block::new_block(data, String::from_utf8(lasthash.to_vec())?, TARGET_HEXT)
            .expect("Failed to create new block");

        let _ = self.db.insert(
            new_block.get_hash(),
            bincode::serialize(&new_block).expect("Failed to insert new block hash"),
        );
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        Ok(())
    }
    pub fn iter(&self) -> BcIter {
        BcIter {
            current_hash: self.current_hash.clone(),
            bc: self,
        }
    }
    pub fn find_unspent_transactions(&self, address: &[u8]) -> Vec<Transaction> {
        let mut spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspend_TXs: Vec<Transaction> = Vec::new();

        for block in self.iter() {
            for tx in block.get_transaction() {
                for idx in 0..tx.vout.len() {
                    if let Some(ids) = spent_TXOs.get(&tx.id) {
                        if ids.contains(&(idx as i32)) {
                            continue;
                        }
                    }
                    if tx.vout[idx].can_be_unlock_with(address) {
                        unspend_TXs.push(tx.to_owned())
                    }
                }
                if !tx.is_coinbase() {
                    for i in &tx.vin {
                        if i.can_unlock_output_with(address) {
                            match spent_TXOs.get_mut(&i.txid) {
                                Some(v) => v.push(i.vout),
                                None => {
                                    spent_TXOs.insert(i.txid.clone(), vec![i.vout]);
                                }
                            }
                        }
                    }
                }
            }
        }

        unspend_TXs
    }
    pub fn find_UTXO(&self) -> HashMap<String, TXOutputs> {
        let mut utxos = HashMap::<String, TXOutputs>::new();
        let spend_TXs = HashMap::<String, Vec<i32>>::new();

        for block in self.iter() {
            for tx in block.get_transaction() {
                for idx in 0..tx.vout.len() {
                    if let Some(ids) = spend_TXs.get(&tx.id) {
                        if ids.contains(&(idx as i32)) {
                            continue;
                        }
                    }

                    match utxos.get_mut(&tx.id) {
                        Some(v) => {
                            v.outputs.push(tx.vout[idx].clone());
                        }
                        None => {
                            utxos.insert(
                                tx.id.clone(),
                                TXOutputs {
                                    outputs: vec![tx.vout[idx].clone()],
                                },
                            );
                        }
                    }
                }
            }
        }
        utxos
    }
    pub fn find_spendable_outputs(
        &self,
        address: &[u8],
        amount: i32,
    ) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated = 0;
        let unspent_TXs = self.find_unspent_transactions(address);

        for tx in unspent_TXs {
            for idx in 0..tx.vout.len() {
                if tx.vout[idx].can_be_unlock_with(address) && accumulated < amount {
                    match unspent_outputs.get_mut(&tx.id) {
                        Some(v) => v.push(idx as i32),
                        None => {
                            unspent_outputs.insert(tx.id.clone(), vec![idx as i32]);
                        }
                    }
                    accumulated += tx.vout[idx].value;

                    if accumulated >= amount {
                        return (accumulated, unspent_outputs);
                    }
                }
            }
        }
        (accumulated, unspent_outputs)
    }
    pub fn find_trans(&self, id: &str) -> Result<Transaction> {
        for b in self.iter() {
            for tx in b.get_transaction() {
                if tx.id == id {
                    return Ok(tx.clone());
                }
            }
        }
        Err(format_err!("Trans not found!"))
    }
    pub fn sign_transaction(&self, tx: &mut Transaction, private_key: &[u8]) -> Result<()> {
        let prev_TXs = self.get_prev_TXs(tx)?;
        tx.sign(private_key, prev_TXs)?;
        Ok(())
    }
    fn get_prev_TXs(&self, tx: &Transaction) -> Result<HashMap<String, Transaction>> {
        let mut prev_TXs = HashMap::new();
        for vin in &tx.vin {
            let prev_TX = self.find_trans(&vin.txid)?;
            prev_TXs.insert(prev_TX.id.clone(), prev_TX);
        }
        Ok(prev_TXs)
    }
}
impl<'a> Iterator for BcIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash) {
            return match encode_block {
                Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    }
}
