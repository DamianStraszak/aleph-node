use crate::data_io::{AlephData, UnvalidatedAlephProposal};
use sp_runtime::traits::Block as BlockT;
use substrate_test_runtime_client::runtime::{Block, Header};

pub fn unvalidated_proposal_from_headers(blocks: Vec<Header>) -> UnvalidatedAlephProposal<Block> {
    let num = blocks.last().unwrap().number;
    let hashes = blocks.into_iter().map(|block| block.hash()).collect();
    UnvalidatedAlephProposal::new(hashes, num)
}

pub fn aleph_data_from_blocks(blocks: Vec<Block>) -> AlephData<Block> {
    let headers = blocks.into_iter().map(|b| b.header().clone()).collect();
    aleph_data_from_headers(headers)
}

pub fn aleph_data_from_headers(headers: Vec<Header>) -> AlephData<Block> {
    if headers.is_empty() {
        AlephData::Empty
    } else {
        AlephData::HeadProposal(unvalidated_proposal_from_headers(headers))
    }
}
