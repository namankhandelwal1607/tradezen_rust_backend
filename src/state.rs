use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
}

#[cw_serde]
pub struct StockCategory {
    pub symbol : String,
    pub stocks : Vec<Addr>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const BOUGHT_STOCKS: Map<Addr, Vec<StockCategory>> = Map::new("bought_stocks");  // Map< Addr : Addr of owner , Addr : Addr of stock NFT>
pub const UNBOUGHT_STOCKS : Item<Vec<StockCategory>> = Item::new("unbought_stocks");
pub const NFT_CODE_ID: Item<u64> = Item::new("nft_code_id");