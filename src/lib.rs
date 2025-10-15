/// Global configuration and state management
pub mod global;
/// Event listeners
pub mod listener;
/// Trade module
pub mod trade;
/// Type module
pub mod types;
/// Wallet module
pub mod wallet;
use crate::types::SuiError;
use crate::types::*;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use reqwest::Client as HttpClient;
use serde_json::Value;
use std::collections::HashMap;
use std::result::Result;

/// Sui network client.
/// # Params
/// - http_client : http client
/// - config : configuration
pub struct SuiClient {
    http_client: HttpClient,
    config: SuiClientConfig,
}

impl SuiClient {
    /// # creates new client
    ///
    /// ## Parameters
    /// - config : client config
    ///
    /// ## Returns
    /// - client object
    ///
    /// ## Example
    /// ```rust
    /// let config = SuiClientConfig {
    ///     rpc_url: mainnet::RPC_URL.to_string(),
    ///     ..Default::default(),
    /// };
    /// let client = SuiClient::new(config);
    /// ```
    pub fn new(config: SuiClientConfig) -> Self {
        Self {
            http_client: HttpClient::new(),
            config,
        }
    }

    /// # create new client by rpc url
    ///
    /// ## Parameters
    /// - url : rpc url
    ///
    /// ## Returns
    /// sui network client
    ///
    /// ## Example
    /// ```rust
    /// let client = SuiClient::new_by_rpc_url(mainnet::RPC_URL.to_string());
    /// ```
    pub fn new_by_rpc_url(url: String) -> Self {
        let config = SuiClientConfig {
            rpc_url: url,
            ..Default::default()
        };
        Self::new(config)
    }

    /// # send JSON request
    ///
    /// ## Parameters
    /// - method: rpc method name
    /// - params: rpc param list
    ///
    /// ## Returns
    /// - Ok(T): Response data
    /// - Err(SuiError): rpc call error
    ///
    /// ## Errors
    /// - SuiError::Rpc: rpc call failed.
    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Vec<Value>,
    ) -> Result<T, SuiError> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: method.to_string(),
            params,
        };
        let response: RpcResponse<T> = self
            .http_client
            .post(&self.config.rpc_url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        if let Some(error) = response.error {
            return Err(SuiError::Rpc(error.message));
        }
        response
            .result
            .ok_or_else(|| SuiError::Rpc("No result in response".to_string()))
    }

    /// # Get object info
    ///
    /// ## Parameters
    /// - object_id : object id
    ///
    /// ## Returns
    /// - Ok(Object) : object information
    /// - Err(SuiError) : error
    ///
    /// ## Example
    /// ```rust
    /// use sui_client::SuiClient;
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = SuiClient::new_by_rpc_url(mainnet::RPC_URL.to_string());
    ///    let object = client.get_object("object id").await.unwrap();
    ///    println!("Object owner: {:?}", object.owner);
    /// }
    /// ```
    pub async fn get_object(&self, object_id: &str) -> Result<Object, SuiError> {
        match self
            .request::<Object>("sui_getObject", vec![object_id.into()])
            .await
        {
            Ok(o) => Ok(o),
            Err(e) => Err(e),
        }
    }

    /// # Get objects owned by address
    ///
    /// ## Parameters
    /// - address : address
    ///
    /// ## Returns
    /// - Ok(Vec<SuiObject>) : all objects owned by this address..
    /// - Err(SuiError) : error
    ///
    /// ## Example
    /// ```rust
    /// use sui_client::SuiClient;
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = SuiClient::new_by_rpc_url(mainnet::RPC_URL.to_string());
    ///    let objects = client.get_objects_owned_by_address("0x123...").await.unwrap();
    ///    for object in objects {
    ///        println!("Object ID: {}", object.object_id);
    ///    }
    /// }
    /// ```
    pub async fn get_objects_owned_by_address(
        &self,
        address: &str,
    ) -> Result<Vec<Object>, SuiError> {
        self.request("sui_getObjectsOwnedByAddress", vec![address.into()])
            .await
    }

    /// # Get coin vec
    ///
    /// ## Parameters
    /// - address: address
    /// - coin_type: coin type ("0x2::sui::SUI")
    ///
    /// ## Returns
    /// - Ok(Vec<Coin>): coin list
    /// - Err(SuiError): error
    ///
    /// ## Example
    /// ```rust
    /// use sui_client::SuiClient;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SuiClient::new_by_rpc_url(mainnet::RPC_URL.to_string());
    ///     // Get SUI coins
    ///     let sui_coins = client.get_coin_vec("0x123...", None).await.unwrap();
    ///     // Get custom coin type
    ///     let custom_coins = client.get_coin_vec("0x123...", Some("0x2::usdc::USDC")).await.unwrap();
    /// }
    /// ```
    pub async fn get_coin_vec(
        &self,
        address: &str,
        coin_type: Option<&str>,
    ) -> Result<Vec<Coin>, SuiError> {
        let coin_type = coin_type.unwrap_or("0x2::sui::SUI");
        self.request("sui_getCoins", vec![address.into(), coin_type.into()])
            .await
    }

    /// # Get balance
    ///
    /// ## Parameters
    /// - address : address
    /// - coin_type : coin type ("0x2::sui::SUI")
    ///
    /// ## Returns
    /// - Ok(u64) : balance
    /// - Err(SuiError) : error
    ///
    /// ## Example
    /// ```rust
    /// use sui_client::SuiClient;
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = SuiClient::new_by_rpc_url(mainnet::RPC_URL.to_string());
    ///   let balance = client.get_balance("0x123...", None).await.unwrap();
    ///   println!("Balance: {} MIST", balance);
    /// }
    /// ```
    pub async fn get_balance(
        &self,
        address: &str,
        coin_type: Option<&str>,
    ) -> Result<u64, SuiError> {
        let coin_type = coin_type.unwrap_or("0x2::sui::SUI");
        let result: HashMap<String, Value> = self
            .request("sui_getBalance", vec![address.into(), coin_type.into()])
            .await?;
        result
            .get("totalBalance")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| SuiError::Rpc("Failed to parse balance".to_string()))
    }

    /// # Execute transaction
    ///
    /// ## Parameters
    /// -  trade_bytes : serialized transaction bytes
    /// -  sign : transaction signature
    /// -  pub_key : public key
    ///
    /// ## Returns
    /// -  Ok(TransactionResponse) : execution transaction result
    /// -  Err(SuiError) : execution transaction error
    ///
    /// ## Example
    /// ```rust
    /// use sui_client::SuiClient;
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = SuiClient::new_by_rpc_url(mainnet::RPC_URL.to_string());
    ///   let tx_bytes = vec![];
    ///   let signature = vec![];
    ///   let pub_key = vec![];
    ///   let response = client.exe_transaction(tx_bytes, signature, pub_key).await.unwrap();
    ///  println!("Transaction digest: {:?}", response.digest);
    /// }
    /// ```
    pub async fn exe_transaction(
        &self,
        trade_bytes: Vec<u8>,
        sign: Vec<u8>,
        pub_key: Vec<u8>,
    ) -> Result<TransactionResponse, SuiError> {
        let tx_bytes = BASE64_STANDARD.encode(trade_bytes);
        let sig_bytes = BASE64_STANDARD.encode(sign);
        let pub_key_bytes = BASE64_STANDARD.encode(pub_key);
        let params = vec![
            tx_bytes.into(),
            "Ed25519".into(),
            sig_bytes.into(),
            pub_key_bytes.into(),
        ];
        self.request("sui_executeTransactionBlock", params).await
    }

    /// # Get trade info
    ///
    /// ## Parameters
    /// - `digest`: trade hash
    ///
    /// ## Returns
    /// - `Ok(TransactionResponse)`: trade details
    /// - `Err(SuiError)`: Query error
    ///
    /// ## Example
    /// ```no_run
    /// use sui_client::SuiClient;
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = SuiClient::new_by_rpc_url(mainnet::RPC_URL.to_string());
    ///    let tx_info = client.get_transaction_info("hash").await.unwrap();
    ///    println!("Transaction status: {:?}", tx_info.effects.status);
    /// }
    /// ```
    pub async fn get_transaction_info(&self, hash: &str) -> Result<TransactionResponse, SuiError> {
        match self.request("sui_getTransaction", vec![hash.into()]).await {
            Ok(tr) => Ok(tr),
            Err(e) => Err(e),
        }
    }
}
