pub mod memory;

use std::cell::RefCell;

use crate::domain::{Metadata, RawWallet, SelfCustodyKey, TransactionLog};

use ic_stable_structures::{BTreeMap as StableBTreeMap, Cell as StableCell, Log as StableLog};
use serde::{Deserialize, Serialize};

use self::memory::Memory;

pub type Timestamp = u64;
pub type RawWalletStable = StableBTreeMap<SelfCustodyKey, RawWallet, Memory>;
pub type TransactionLogStable = StableLog<TransactionLog, Memory, Memory>;
// pub type TransactionLedgerStable = StableLog<TransactionLedger, Memory, Memory>;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());

}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_metadata")]
    pub metadata: StableCell<Metadata, Memory>,
    #[serde(skip, default = "init_stable_counter")]
    pub counter: StableCell<u128, Memory>,
    #[serde(skip, default = "init_stable_wallet")]
    pub wallets: RawWalletStable,
    #[serde(skip, default = "init_stable_transaction_log")]
    pub logs: TransactionLogStable,
    // pub ledger: TransactionLedgerStable,
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_stable_metadata(),
            counter: init_stable_counter(),
            wallets: init_stable_wallet(),
            logs: init_stable_transaction_log(),
        }
    }
}

fn init_stable_metadata() -> StableCell<Metadata, Memory> {
    StableCell::init(memory::get_metadata_memory(), Metadata::default())
        .expect("failed to initialize the metadata cell")
}

fn init_stable_counter() -> StableCell<u128, Memory> {
    StableCell::init(memory::get_counter_memory(), 0u128)
        .expect("Could not initialize sig count memory")
}

fn init_stable_wallet() -> RawWalletStable {
    StableBTreeMap::init(memory::get_wallet_memory())
}

fn init_stable_transaction_log() -> TransactionLogStable {
    StableLog::init(
        memory::get_transaction_log_index_memory(),
        memory::get_transaction_log_data_memory(),
    )
    .expect("failed to init wallet ledger")
}