use super::range_manager::{Range, RangeManager};
use crate::data_structure::transaction::Transaction;

pub struct StateProcessor {}

impl StateProcessor {
    pub fn apply(input_ranges: &[Range], transaction: &Transaction) -> Vec<Range> {
        let state_update = transaction.get_state_update();
        // check transaction witness
        // transaction.get_transaction_witness()
        // call verify_deprecation
        RangeManager::put(
            input_ranges,
            state_update.get_start(),
            state_update.get_end(),
            &rlp::encode(state_update),
        )
    }
}