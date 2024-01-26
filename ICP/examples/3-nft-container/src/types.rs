use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::num::TryFromIntError;
use std::result::Result as StdResult;

use candid::{CandidType, Principal};
use ic_cdk::export::candid;
use ic_certified_map::Hash;

#[derive(CandidType, Deserialize, Clone)]
pub struct LogoResult {
    pub logo_type: Cow<'static, str>,
    pub data: Cow<'static, str>,
}

#[derive(CandidType, Deserialize)]
pub struct StableState {
    pub state: State,
    pub hashes: Vec<(String, Hash)>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct State {
    pub nfts: Vec<Nft>,
    pub custodians: HashSet<Principal>,
    pub operators: HashMap<Principal, HashSet<Principal>>, // owner to operators
    pub logo: Option<LogoResult>,
    pub name: String,
    pub symbol: String,
    pub txid: u128,
}

#[derive(CandidType, Deserialize)]
pub struct Nft {
    pub owner: Principal,
    pub approved: Option<Principal>,
    pub id: u64,
    pub metadata: MetadataDesc,
    pub content: Vec<u8>,
}

pub type MetadataDesc = Vec<MetadataPart>;
pub type MetadataDescRef<'a> = &'a [MetadataPart];

#[derive(CandidType, Deserialize)]
pub struct MetadataPart {
    pub purpose: MetadataPurpose,
    pub key_val_data: HashMap<String, MetadataVal>,
    pub data: Vec<u8>,
}

#[derive(CandidType, Deserialize, PartialEq)]
pub enum MetadataPurpose {
    Preview,
    Rendered,
}

#[derive(CandidType, Deserialize)]
pub struct MintResult {
    pub token_id: u64,
    pub id: u128,
}

#[allow(clippy::enum_variant_names)]
#[derive(CandidType, Deserialize)]
pub enum MetadataVal {
    TextContent(String),
    BlobContent(Vec<u8>),
    NatContent(u128),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
}

impl State {
    pub fn next_txid(&mut self) -> u128 {
        let txid = self.txid;
        self.txid += 1;
        txid
    }
}

#[derive(CandidType, Deserialize)]
pub enum InterfaceId {
    Approval,
    TransactionHistory,
    Mint,
    Burn,
    TransferNotification,
}

#[derive(CandidType, Deserialize)]
pub enum ConstrainedError {
    Unauthorized,
}

#[derive(CandidType)]
pub struct ExtendedMetadataResult<'a> {
    pub metadata_desc: MetadataDescRef<'a>,
    pub token_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub custodians: Option<HashSet<Principal>>,
    pub logo: Option<LogoResult>,
    pub name: String,
    pub symbol: String,
}

#[derive(CandidType, Deserialize)]
pub enum Error {
    Unauthorized,
    InvalidTokenId,
    ZeroAddress,
    Other,
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self::InvalidTokenId
    }
}

pub type Result<T = u128, E = Error> = StdResult<T, E>;
