pub mod mainnet {
    pub const RPC_URL: &str = "https://fullnode.mainnet.sui.io:443";
    pub const WSS_URL: &str = "wss://fullnode.mainnet.sui.io:443";
}
pub mod testnet {
    pub const RPC_URL: &str = "https://fullnode.testnet.sui.io:443";
    pub const WSS_URL: &str = "wss://fullnode.testnet.sui.io:443";
}
pub mod devnet {
    pub const RPC_URL: &str = "https://fullnode.devnet.sui.io:443";
    pub const WSS_URL: &str = "wss://fullnode.devnet.sui.io:443";
    pub const FAUCET_URL: &str = "https://faucet.devnet.sui.io/gas";
}
