extern crate ethereum_types;
extern crate rlp;

use super::transaction::Transaction;
use ethereum_types::H256;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

#[derive(Clone, Debug)]
pub struct Block {
    transactions: Vec<Transaction>,
    root: H256,
}

impl Block {
    pub fn new(transactions: &[Transaction], root: H256) -> Block {
        Block {
            transactions: transactions.to_vec(),
            root,
        }
    }
}

impl Encodable for Block {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append_list(&self.transactions);
        s.append(&self.root);
    }
}

impl Decodable for Block {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let transactions: Vec<Transaction> = rlp.list_at(0)?;
        let root: H256 = rlp.val_at(1)?;
        Ok(Block { transactions, root })
    }
}

#[cfg(test)]
mod tests {
    use super::Block;
    use ethereum_types::H256;

    #[test]
    fn test_new() {
        let block = Block::new(&[], H256::zero());
        assert_eq!(block.root, H256::zero());
    }

    #[test]
    fn test_rlp_encode() {
        let block = Block::new(&[], H256::zero());
        let encoded = rlp::encode(&block);
        let _decoded: Block = rlp::decode(&encoded).unwrap();
        assert_eq!(_decoded.root, block.root);
    }

}
