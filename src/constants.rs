use lazy_static::lazy_static;
use std::{collections::HashMap, sync::RwLock};

pub const MAX_NAME_LENGTH: usize = 32;
pub const MAX_URI_LENGTH: usize = 200;
pub const MAX_SYMBOL_LENGTH: usize = 10;
pub const MAX_CREATOR_LEN: usize = 32 + 1 + 1;

pub const METADATA_PREFIX: &str = "metadata";
pub const MASTER_EDITION_PREFIX: &str = "edition";
pub const USER_PREFIX: &str = "user";

pub const METAPLEX_PROGRAM_ID: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
pub const CANDY_MACHINE_PROGRAM_ID: &str = "cndyAnrLdpjq1Ssp1z8xxDsB8dxe7u4HL5Nxi2K5WXZ";

pub const PUBLIC_RPC_URLS: &[&str] = &[
    "https://api.mainnet-beta.solana.com",
    "https://solana-api.projectserum.com",
    "https://small-white-breeze.solana-mainnet.quiknode.pro/d33b740087d0858f0b2464a71f6ebf2111a1f122/"
];

pub const DEFAULT_RPC_DELAY_MS: u32 = 200;

lazy_static! {
    pub static ref USE_RATE_LIMIT: RwLock<bool> = RwLock::new(false);
    pub static ref RPC_DELAY_NS: RwLock<u32> = RwLock::new(DEFAULT_RPC_DELAY_MS * 1_000_000);
    pub static ref RATE_LIMIT_DELAYS: HashMap<&'static str, u32> =
        [("https://ssc-dao.genesysgo.net", 25),]
            .iter()
            .copied()
            .collect();
}
