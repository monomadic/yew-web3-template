// use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Config {
    pub chain_id: String,
    pub chain_name: String,
    pub rpc_url: String,
    pub base_currency_decimals: u32,
    pub base_currency_name: String,
    pub base_currency_symbol: String,
    pub block_explorer_url: String,
}

impl Config {
    pub fn load() -> Self {
        Config {
            chain_id: option_env!("CHAIN_ID").unwrap_or("").to_string(),
            chain_name: option_env!("CHAIN_NAME").unwrap_or("").to_string(),
            rpc_url: option_env!("RPC_URL").unwrap_or("").to_string(),
            base_currency_decimals: option_env!("BASE_CURRENCY_DECIMALS")
                .unwrap_or("2")
                .parse::<u32>()
                .unwrap(),
            base_currency_name: option_env!("BASE_CURRENCY_NAME").unwrap_or("").to_string(),
            base_currency_symbol: option_env!("BASE_CURRENCY_SYMBOL")
                .unwrap_or("")
                .to_string(),
            block_explorer_url: option_env!("BLOCK_EXPLORER_URL").unwrap_or("").to_string(),
        }
    }
}

impl Into<yew_ethereum_provider::Chain> for Config {
    fn into(self) -> yew_ethereum_provider::Chain {
        yew_ethereum_provider::Chain {
            chain_id: self.chain_id, // hex
            chain_name: self.chain_name,
            rpc_urls: [self.rpc_url.into()],
            native_currency: yew_ethereum_provider::BaseCurrency {
                decimals: self.base_currency_decimals,
                name: self.base_currency_name,
                symbol: self.base_currency_symbol,
            },
            block_explorer_urls: Some([self.block_explorer_url]),
        }
    }
}
