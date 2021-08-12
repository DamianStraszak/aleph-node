use serde::{Deserialize, Serialize};
use serde_json::Result;
pub(crate) const HARD_FORK_PATH: &str = "/tmp/fork_history";
use log::{debug, error};
use sc_client_api::{Backend, UsageProvider};
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use std::{fmt::Debug, sync::Arc};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct HardFork {
    block_num: u32,
    bad_blocks: Vec<String>,
}

fn read_hard_forks() -> Result<Vec<HardFork>> {
    let hard_forks_data = std::fs::read_to_string(HARD_FORK_PATH).unwrap();
    let mut hard_forks: Vec<HardFork> = serde_json::from_str(&hard_forks_data)?;
    hard_forks.sort();
    Ok(hard_forks)
}

fn _to_file(hard_forks: Vec<HardFork>) {
    if let Ok(hard_forks_json) = serde_json::to_string(&hard_forks) {
        std::fs::write(HARD_FORK_PATH, &hard_forks_json).unwrap();
    }
}

pub fn load_hard_forks<B, BE, C>(client: Arc<C>, backend: Arc<BE>)
where
    B: BlockT,
    BE: Backend<B>,
    C: UsageProvider<B> + HeaderBackend<B>,
    //<<<B as BlockT>::Header as HeaderT>::Number as FromStr>::Err: Debug,
    <<B as BlockT>::Header as HeaderT>::Number: From<u32>,
{
    let hard_forks = read_hard_forks().unwrap();
    if hard_forks.len() > 0 {
        let best_number = client.info().best_number;
        let fork_at = hard_forks[0].block_num.into();
        debug!(target: "afa", "best_number: {:?} fork_at: {:?}", best_number, fork_at);
        let blocks = best_number - fork_at;
        debug!(target: "afa", "best_number: {:?} fork_at: {:?} blocks: {:?}", best_number, fork_at, blocks);
        // WARNING: the "true" below means that we also revert finalized blocks if necessary
        // this is marked as unsafe in substrate
        if let Ok(reverted) = backend.revert(blocks, true) {
            debug!(target: "afa", "Reverted chain succesfully ({:?} blocks)", reverted.0);
        } else {
            error!(target: "afa", "Failed to revert chain");
        }
    } else {
        debug!(target: "afa", "No hard forks found in the config file");
    }
}

//let example = vec![ HardFork {block_num: 10, bad_blocks: vec![String::from("dsgdsg"), String::from("dsgsdgsd")]}];
//to_file(example);
