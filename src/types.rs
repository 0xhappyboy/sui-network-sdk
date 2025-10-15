use serde::{Deserialize, Serialize};
use std::fmt;

use crate::global::devnet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiClientConfig {
    pub rpc_url: String,
    pub wss_url: String,
    pub faucet_url: String,
}

impl Default for SuiClientConfig {
    fn default() -> Self {
        Self {
            rpc_url: devnet::RPC_URL.to_string(),
            wss_url: devnet::WSS_URL.to_string(),
            faucet_url: devnet::FAUCET_URL.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub object_id: String,
    pub version: u64,
    pub digest: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub owner: Owner,
    pub previous_transaction: String,
    pub data: ObjectData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub address_owner: Option<String>,
    pub object_owner: Option<String>,
    pub shared: Option<SharedOwner>,
    pub immutable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedOwner {
    pub initial_shared_version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectData {
    pub data_type: String,
    pub fields: serde_json::Value,
    pub has_public_transfer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub digest: String,
    pub effects: TransactionEffects,
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEffects {
    pub status: ExecutionStatus,
    pub gas_used: GasCostSummary,
    pub transaction_digest: String,
    pub mutated: Vec<OwnedObjectRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStatus {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasCostSummary {
    pub computation_cost: u64,
    pub storage_cost: u64,
    pub storage_rebate: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedObjectRef {
    pub owner: Owner,
    pub reference: ObjectRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRef {
    pub object_id: String,
    pub version: u64,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    #[serde(rename = "type")]
    pub event_type: String,
    pub parsed_json: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventId {
    pub tx_digest: String,
    pub event_seq: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coin {
    pub coin_object_id: String,
    pub version: u64,
    pub digest: String,
    pub balance: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub result: Option<T>,
    pub error: Option<RpcError>,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug)]
pub enum SuiError {
    HttpRequest(String),
    WebSocket(String),
    Json(String),
    Hex(String),
    Base64(String),
    InvalidPrivateKey,
    Rpc(String),
    Transaction(String),
    Io(String),
    CallContract(String),
    Gas(String),
    Sign(String),
}

impl fmt::Display for SuiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SuiError::HttpRequest(e) => write!(f, "HTTP request failed: {}", e),
            SuiError::WebSocket(e) => write!(f, "WebSocket error: {}", e),
            SuiError::Json(e) => write!(f, "JSON error: {}", e),
            SuiError::Hex(e) => write!(f, "Hex error: {}", e),
            SuiError::Base64(e) => write!(f, "Base64 error: {}", e),
            SuiError::InvalidPrivateKey => write!(f, "Invalid private key"),
            SuiError::Rpc(e) => write!(f, "RPC error: {}", e),
            SuiError::Transaction(e) => write!(f, "Transaction error: {}", e),
            SuiError::Io(e) => write!(f, "IO error: {}", e),
            SuiError::CallContract(e) => write!(f, "Call Contract error: {}", e),
            SuiError::Gas(e) => write!(f, "Gas error: {}", e),
            SuiError::Sign(e) => write!(f, "Sign error: {}", e),
        }
    }
}

impl std::error::Error for SuiError {}

impl From<reqwest::Error> for SuiError {
    fn from(err: reqwest::Error) -> Self {
        SuiError::HttpRequest(err.to_string())
    }
}

impl From<serde_json::Error> for SuiError {
    fn from(err: serde_json::Error) -> Self {
        SuiError::Json(err.to_string())
    }
}

impl From<hex::FromHexError> for SuiError {
    fn from(err: hex::FromHexError) -> Self {
        SuiError::Hex(err.to_string())
    }
}

impl From<base64::DecodeError> for SuiError {
    fn from(err: base64::DecodeError) -> Self {
        SuiError::Base64(err.to_string())
    }
}

impl From<std::io::Error> for SuiError {
    fn from(err: std::io::Error) -> Self {
        SuiError::Io(err.to_string())
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for SuiError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        SuiError::WebSocket(err.to_string())
    }
}

impl From<url::ParseError> for SuiError {
    fn from(err: url::ParseError) -> Self {
        SuiError::WebSocket(err.to_string())
    }
}
