pub(crate) mod asset_math;

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("Builder is incomplete")]
    Incomplete,
    #[error("Conflicting requirement")]
    Conflicting,
    #[error("UTxO decoding failed")]
    UtxoDecode,
    #[error("UTxO set is empty")]
    EmptyUtxoSet,
    #[error("Transaction has no inputs")]
    MalformedScript,
    #[error("Could not decode datum bytes")]
    MalformedDatum,
    #[error("Invalid bytes length for datum hash")]
    MalformedDatumHash,
    #[error("Input/policy pointed to by redeemer not found in tx")]
    RedeemerTargetMissing,
    #[error("Invalid network ID")]
    InvalidNetworkId,
    #[error("Corrupted transaction bytes in built transaction")]
    CorruptedTxBytes,
    #[error("Public key for private key is malformed")]
    MalformedKey,
    #[error("Asset name must be 32 bytes or less")]
    AssetNameTooLong,
    #[error("Invalid asset policy id hex")]
    MalformedAssetPolicyIdHex,
    #[error("Malformed TxoRef")]
    MalformedTxoRef,
    #[error("Ledger error: {0}")]
    LedgerError(String),
}

impl From<crate::wit::balius::app::ledger::LedgerError> for BuildError {
    fn from(err: crate::wit::balius::app::ledger::LedgerError) -> Self {
        BuildError::LedgerError(err.to_string())
    }
}

pub use pallas_codec as codec;
pub use pallas_primitives::babbage as primitives;

pub struct PParams {
    pub min_fee_a: u64,
    pub min_fee_b: u64,
    pub min_utxo_value: u64,
}

pub trait Ledger {
    fn read_utxos(&self, refs: &[dsl::TxoRef]) -> Result<dsl::UtxoSet, BuildError>;
    fn search_utxos(&self, pattern: &dsl::UtxoPattern) -> Result<dsl::UtxoSet, BuildError>;
}

pub struct BuildContext {
    pub network: primitives::NetworkId,
    pub pparams: PParams,
    pub estimated_fee: u64,
    pub ledger: Box<dyn Ledger>,

    pub tx_body: Option<primitives::TransactionBody>,
}

mod build;
mod dsl;

pub use build::*;
pub use dsl::*;
