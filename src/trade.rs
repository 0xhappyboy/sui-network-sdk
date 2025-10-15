use crate::SuiClient;
use crate::types::SuiError;
use crate::wallet::Wallet;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde_json::Value;

pub struct Trade<'a> {
    client: &'a SuiClient,
    wallet: &'a Wallet,
    gas_payment: Option<String>,
    gas_budget: u64,
}

impl<'a> Trade<'a> {
    pub fn new(client: &'a SuiClient, wallet: &'a Wallet) -> Self {
        Self {
            client,
            wallet,
            gas_payment: None,
            gas_budget: 1000,
        }
    }
    pub fn with_gas_payment(mut self, gas_payment: String) -> Self {
        self.gas_payment = Some(gas_payment);
        self
    }
    pub fn with_gas_budget(mut self, gas_budget: u64) -> Self {
        self.gas_budget = gas_budget;
        self
    }
    /// transfer by sui
    pub async fn transfer_by_sui(
        &self,
        recipient: &str,
        amount: u64,
    ) -> Result<(Vec<u8>, Vec<u8>), SuiError> {
        let gas_payment = self
            .get_gas_payment()
            .await
            .ok_or_else(|| SuiError::Transaction("No gas payment available".to_string()))?;
        let params = vec![
            self.wallet.address.clone().into(),
            gas_payment.into(),
            amount.to_string().into(),
            recipient.into(),
            self.gas_budget.to_string().into(),
        ];
        let transaction_data: Value = self.client.request("unsafe_transferObject", params).await?;
        self.sign_transaction(transaction_data).await
    }
    // call contract function
    pub async fn call_contract_function(
        &self,
        package_object_id: &str,
        module: &str,
        function: &str,
        type_arguments: Vec<&str>,
        arguments: Vec<Value>,
    ) -> Result<(Vec<u8>, Vec<u8>), SuiError> {
        let gas_payment = self
            .get_gas_payment()
            .await
            .ok_or_else(|| SuiError::CallContract("No gas payment available".to_string()))?;
        let params = vec![
            self.wallet.address.clone().into(),
            package_object_id.into(),
            module.into(),
            function.into(),
            type_arguments
                .into_iter()
                .map(Value::from)
                .collect::<Vec<_>>()
                .into(),
            arguments.into(),
            gas_payment.into(),
            self.gas_budget.to_string().into(),
        ];
        let transaction_data: Value = self.client.request("unsafe_moveCall", params).await?;
        self.sign_transaction(transaction_data).await
    }
    // merge coins
    pub async fn merge_coins(
        &self,
        primary_coin: &str,
        coin_to_merge: &str,
    ) -> Result<(Vec<u8>, Vec<u8>), SuiError> {
        let gas_payment = self
            .get_gas_payment()
            .await
            .ok_or_else(|| SuiError::CallContract("No gas payment available".to_string()))?;
        let params = vec![
            self.wallet.address.clone().into(),
            primary_coin.into(),
            coin_to_merge.into(),
            gas_payment.into(),
            self.gas_budget.to_string().into(),
        ];
        let transaction_data: Value = self.client.request("unsafe_mergeCoins", params).await?;
        self.sign_transaction(transaction_data).await
    }
    // split coin
    pub async fn split_coin(
        &self,
        coin_object_id: &str,
        split_amounts: Vec<u64>,
    ) -> Result<(Vec<u8>, Vec<u8>), SuiError> {
        let gas_payment = self
            .get_gas_payment()
            .await
            .ok_or_else(|| SuiError::CallContract("No gas payment available".to_string()))?;
        let amounts: Vec<Value> = split_amounts
            .into_iter()
            .map(|amount| amount.to_string().into())
            .collect();
        let params = vec![
            self.wallet.address.clone().into(),
            coin_object_id.into(),
            amounts.into(),
            gas_payment.into(),
            self.gas_budget.to_string().into(),
        ];
        let transaction_data: Value = self.client.request("unsafe_splitCoin", params).await?;
        self.sign_transaction(transaction_data).await
    }
    /// get gas payment
    async fn get_gas_payment(&self) -> Option<String> {
        if let Some(ref gas_payment) = self.gas_payment {
            return Some(gas_payment.clone());
        }
        match self.client.get_coin_vec(&self.wallet.address, None).await {
            Ok(coins) => coins.first().map(|coin| coin.coin_object_id.clone()),
            Err(_) => None,
        }
    }
    /// sign transaction
    async fn sign_transaction(
        &self,
        transaction_data: Value,
    ) -> Result<(Vec<u8>, Vec<u8>), SuiError> {
        let tx_bytes_str = transaction_data
            .get("txBytes")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SuiError::Transaction("No txBytes in response".to_string()))?;
        let tx_bytes = BASE64_STANDARD
            .decode(tx_bytes_str)
            .map_err(|e| SuiError::Sign(format!("Failed to decode txBytes: {}", e)))?;
        // sign transaction
        let signature = self.wallet.sign(&tx_bytes);
        Ok((tx_bytes, signature))
    }
}
