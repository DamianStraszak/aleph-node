use aleph_client::{
    account_from_keypair, keypair_from_string, BlockNumber, Connection, VestingSchedule,
};
use log::{error, info};
use primitives::{Balance, TOKEN};

/// Delegates to `aleph_client::vest`.
///
/// `connection` should be signed: the vesting is performed for the signer.
pub fn vest(connection: Connection) {
    match aleph_client::vest(connection) {
        Ok(_) => info!("Vesting has succeeded"),
        Err(e) => error!("Vesting has failed with:\n {:?}", e),
    }
}

/// Delegates to `aleph_client::vest_other`.
///
/// `connection` should be signed: the vesting is performed by the signer for
/// `vesting_account_seed`.
pub fn vest_other(connection: Connection, vesting_account_seed: String) {
    let vester = account_from_keypair(&keypair_from_string(vesting_account_seed.as_str()));
    match aleph_client::vest_other(connection, vester) {
        Ok(_) => info!("Vesting on behalf has succeeded"),
        Err(e) => error!("Vesting on behalf has failed with:\n {:?}", e),
    }
}

/// Delegates to `aleph_client::vested_transfer`.
///
/// `connection` should be signed: the transfer is performed from the signer to `target_seed`.
/// `amount_in_tokens`, `per_block` and `starting_block` corresponds to the fields of
/// `aleph_client::VestingSchedule` struct.
pub fn vested_transfer(
    connection: Connection,
    target_seed: String,
    amount_in_tokens: u64,
    per_block: Balance,
    starting_block: BlockNumber,
) {
    let receiver = account_from_keypair(&keypair_from_string(target_seed.as_str()));
    let schedule =
        VestingSchedule::new(amount_in_tokens as u128 * TOKEN, per_block, starting_block);
    match aleph_client::vested_transfer(connection, receiver, schedule) {
        Ok(_) => info!("Vested transfer has succeeded"),
        Err(e) => error!("Vested transfer has failed with:\n {:?}", e),
    }
}
