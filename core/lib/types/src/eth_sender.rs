use serde::{Deserialize, Serialize};

use crate::{aggregated_operations::AggregatedActionType, Address, Nonce, H256};

#[derive(Clone, Deserialize, Serialize)]
pub enum EthTxBlobSidecar {
    EthTxBlobSidecarV1(EthTxBlobSidecarV1),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SidecarBlob {
    pub blob: Vec<u8>,
    pub commitment: Vec<u8>,
    pub proof: Vec<u8>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EthTxBlobSidecarV1 {
    pub blobs: Vec<SidecarBlob>,
}

#[derive(Clone)]
pub struct EthTx {
    pub id: u32,
    pub nonce: Nonce,
    pub contract_address: Address,
    pub raw_tx: Vec<u8>,
    pub tx_type: AggregatedActionType,
    pub created_at_timestamp: u64,
    pub predicted_gas_cost: u64,
    pub blob_sidecar: Option<EthTxBlobSidecar>,
}

impl std::fmt::Debug for EthTx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Do not print `raw_tx`
        f.debug_struct("EthTx")
            .field("id", &self.id)
            .field("nonce", &self.nonce)
            .field("contract_address", &self.contract_address)
            .field("tx_type", &self.tx_type)
            .field("created_at_timestamp", &self.created_at_timestamp)
            .field("predicted_gas_cost", &self.predicted_gas_cost)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct TxHistory {
    pub id: u32,
    pub eth_tx_id: u32,
    pub base_fee_per_gas: u64,
    pub priority_fee_per_gas: u64,
    pub tx_hash: H256,
    pub signed_raw_tx: Vec<u8>,
    pub sent_at_block: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct TxHistoryToSend {
    pub id: u32,
    pub eth_tx_id: u32,
    pub base_fee_per_gas: u64,
    pub priority_fee_per_gas: u64,
    pub tx_hash: H256,
    pub signed_raw_tx: Vec<u8>,
    pub nonce: Nonce,
}
