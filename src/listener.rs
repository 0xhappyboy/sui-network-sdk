use crate::types::SuiError;
use futures::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// # Sui Network Listener
///
/// Use WebSocket real-time monitoring capabilities for Sui blockchain events, Supports transaction tracking, event monitoring, and address-specific notifications.
///
/// ## Example
/// ```rust
/// use sui_client::listener::Listener;
///
/// #[tokio::main]
/// async fn main() {
///     let listener = Listener::new(mainnet::WSS_URL.to_string());
///     listener.listen_transactions(|tx_digest| {
///         // new transactions
///     }).await.unwrap();
/// }
/// ```
pub struct Listener {
    pub url: String, // websocket url
}

impl Listener {
    /// # create listener
    ///
    /// ## Parameters
    /// - url : websocket url
    ///
    /// ## Returns
    /// listener
    ///
    /// ## Example
    /// ```no_run
    /// use sui_client::listener::Listener;
    /// let listener = Listener::new(mainnet::WSS_URL.to_string());
    /// ```
    pub fn new(url: String) -> Self {
        Self { url }
    }

    /// # Listen transactions
    ///
    /// ## Parameters
    /// - callback: call back function
    ///
    /// ## Returns
    /// - Ok(()) : listening successfully.
    /// - Err(SuiError) : WebSocket error
    ///
    /// ## Example
    /// ```rust
    /// use sui_client::listener::Listener;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let listener = Listener::new(mainnet::WSS_URL.to_string());
    /// listener.listen_transactions(|tx_digest| {
    ///     // new transactions
    /// }).await.unwrap();
    /// }
    /// ```
    pub async fn listen_transactions<F>(&self, mut callback: F) -> Result<(), SuiError>
    where
        F: FnMut(String),
    {
        let (ws_stream, _) = connect_async(&self.url).await?;
        let (mut write, mut read) = ws_stream.split();
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sui_subscribeTransaction",
            "params": [{"All": []}]
        });
        write.send(Message::Text(msg.to_string().into())).await?;
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Ok(event) = serde_json::from_str::<Value>(&text) {
                        if let Some(tx_digest) = event
                            .get("params")
                            .and_then(|p| p.get("result"))
                            .and_then(|r| r.get("digest"))
                            .and_then(|d| d.as_str())
                        {
                            // new transactions
                            callback(tx_digest.to_string());
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    break;
                }
                Err(e) => {
                    return Err(SuiError::WebSocket(e.to_string()));
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// # Listen all events
    ///
    /// ## Parameters
    /// - callback : callback function
    ///
    /// ## Returns
    /// - Ok(()) : Listening Successfully.
    /// - Err(SuiError) : WebSocket Error.
    ///
    /// ## Example
    /// ```rust
    /// use sui_client::listener::Listener;
    /// #[tokio::main]
    /// async fn main() {
    /// let listener = Listener::new(mainnet::WSS_URL.to_string());
    /// listener.listen_events(|event| {
    ///       // new event
    /// }).await.unwrap();
    /// }
    /// ```
    pub async fn listen_events<F>(&self, mut callback: F) -> Result<(), SuiError>
    where
        F: FnMut(Value),
    {
        let (ws_stream, _) = connect_async(&self.url).await?;
        let (mut write, mut read) = ws_stream.split();
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sui_subscribeEvent",
            "params": [{"All": []}]
        });
        write.send(Message::Text(msg.to_string().into())).await?;
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Ok(event) = serde_json::from_str::<Value>(&text) {
                        // new event
                        callback(event);
                    }
                }
                Ok(Message::Close(_)) => {
                    break;
                }
                Err(e) => {
                    return Err(SuiError::WebSocket(e.to_string()));
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// # Listen transactions by address
    ///
    /// ## Parameters
    /// - address : address.
    /// - callback : callback function
    ///
    /// ## Returns
    /// - Ok(()) : Listening Successfully.
    /// - Err(SuiError) : WebSocket Error.
    ///
    /// ## Note
    /// This method does not care whether the address is a sender or a receiver.
    ///
    /// ## Example
    /// ```rust
    /// # use sui_client::listener::Listener;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let listener = Listener::new(mainnet::WSS_URL.to_string());
    /// let address = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    ///
    /// listener.listen_address_transactions(address, |tx_digest| {
    ///      // this address new transaction
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn listen_address_transactions<F>(
        &self,
        address: &str,
        mut callback: F,
    ) -> Result<(), SuiError>
    where
        F: FnMut(String),
    {
        let (ws_stream, _) = connect_async(&self.url).await?;
        let (mut write, mut read) = ws_stream.split();
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sui_subscribeTransaction",
            "params": [{"ToOrFromAddress": {"addr": address}}]
        });
        write.send(Message::Text(msg.to_string().into())).await?;
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Ok(event) = serde_json::from_str::<Value>(&text) {
                        if let Some(tx_digest) = event
                            .get("params")
                            .and_then(|p| p.get("result"))
                            .and_then(|r| r.get("digest"))
                            .and_then(|d| d.as_str())
                        {
                            callback(tx_digest.to_string());
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    break;
                }
                Err(e) => {
                    return Err(SuiError::WebSocket(e.to_string()));
                }
                _ => {}
            }
        }
        Ok(())
    }
}
