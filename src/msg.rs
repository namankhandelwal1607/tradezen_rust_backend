use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

#[cw_serde]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub code_id : u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    MintStock {
        token_name : String,
        token_symbol : String, 
        price_per_share : Uint64,
        stocks : Uint64,
    },
    BuyStock {
        token_symbol : String,
        stock_address : Addr,
    },
    SellStock {
        token_symbol : String,
        stock_address : Addr,
    },
}

#[cw_serde]
pub enum QueryMsg {
    QueryDetails{ stock_address : Addr},
    BoughtStocks{ address : Addr },
    UnboughtStocks {},
}

#[cw_serde]
pub struct AllStocksResponse {
    pub stocks: Vec<SymbolStock>,
}

#[cw_serde]
pub struct SymbolStock{
    pub symbol : String,
    pub stock_info : Vec<StockInfo>,
}


#[cw_serde]
pub struct StockInfo{
    pub address : Addr,
    pub contract_info : ContractInfo,
}

#[cw_serde]
pub struct ContractInfo{
    pub token_name : String,
    pub token_symbol : String,
    pub price_per_share : Uint64,
    pub stocks : Uint64,
}