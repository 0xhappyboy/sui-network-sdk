use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiClientConfig {
    pub rpc_url: String,
    pub ws_url: String,
    pub faucet_url: String,
}

impl Default for SuiClientConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://fullnode.devnet.sui.io:443".to_string(),
            ws_url: "wss://fullnode.devnet.sui.io:443".to_string(),
            faucet_url: "https://faucet.devnet.sui.io/gas".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiObject {
    pub object_id: String,
    pub version: u64,
    pub digest: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub owner: SuiOwner,
    pub previous_transaction: String,
    pub storage_rebate: Option<u64>,
    pub data: SuiObjectData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiOwner {
    pub AddressOwner: Option<String>,
    pub ObjectOwner: Option<String>,
    pub Shared: Option<SharedOwner>,
    pub Immutable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedOwner {
    pub initial_shared_version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiObjectData {
    pub data_type: String,
    pub fields: serde_json::Value,
    pub has_public_transfer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiTransactionResponse {
    pub digest: String,
    pub raw_transaction: String,
    pub effects: SuiTransactionEffects,
    pub events: Vec<SuiEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiTransactionEffects {
    pub status: SuiExecutionStatus,
    pub gas_used: SuiGasCostSummary,
    pub shared_objects: Vec<SuiObjectRef>,
    pub transaction_digest: String,
    pub mutated: Vec<SuiOwnedObjectRef>,
    pub created: Vec<SuiOwnedObjectRef>,
    pub deleted: Vec<SuiObjectRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiExecutionStatus {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiGasCostSummary {
    pub computation_cost: u64,
    pub storage_cost: u64,
    pub storage_rebate: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiObjectRef {
    pub object_id: String,
    pub version: u64,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiOwnedObjectRef {
    pub owner: SuiOwner,
    pub reference: SuiObjectRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiEvent {
    pub id: SuiEventId,
    pub package_id: String,
    pub transaction_module: String,
    pub sender: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub parsed_json: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiEventId {
    pub tx_digest: String,
    pub event_seq: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coin {
    pub coin_type: String,
    pub coin_object_id: String,
    pub version: u64,
    pub digest: String,
    pub balance: u64,
    pub locked_until_epoch: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResult<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: u64,
}

pub type SuiResult<T> = Result<T, SuiError>;

#[derive(Debug, Clone)]
pub struct SuiError {
    pub message: String,
    pub code: i32,
}

impl std::fmt::Display for SuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuiError {}: {}", self.code, self.message)
    }
}

impl std::error::Error for SuiError {}