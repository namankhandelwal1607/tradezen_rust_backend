#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, WasmQuery, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64, SubMsg, WasmMsg, ReplyOn, Reply};
use cw2::set_contract_version;
use cosmwasm_schema::cw_serde;

use cw_utils::parse_reply_instantiate_data;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ContractInfo, AllStocksResponse};
use crate::state::{Config,  StockCategory, BOUGHT_STOCKS, CONFIG, NFT_CODE_ID, UNBOUGHT_STOCKS};

const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_TOKEN_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = msg.admin.unwrap_or(info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;

    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;

    let unbought_stocks = vec![];
    UNBOUGHT_STOCKS.save(deps.storage, &unbought_stocks)?;

    NFT_CODE_ID.save(deps.storage, &msg.code_id)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MintStock { token_name, token_symbol, price_per_share, stocks } => execute::mint_stock(deps, token_name, token_symbol, price_per_share, stocks),
        ExecuteMsg::BuyStock { token_symbol, stock_address } => execute::buy_stock(deps, info, token_symbol, stock_address),
        ExecuteMsg::SellStock { token_symbol, stock_address } => execute::sell_stock(deps, info, token_symbol, stock_address),
    }
}

pub mod execute{
    use crate::state::StockCategory;

    use super::*;

    #[cw_serde]
    pub struct MintMsg {
    pub token_name : String,
    pub token_symbol: String,
    pub price_per_share: Uint64,
    pub stocks: Uint64,
}

    pub fn mint_stock(deps : DepsMut, token_name : String, token_symbol : String, price_per_share : Uint64, stocks : Uint64) -> Result<Response, ContractError>{

        let nft_msg : SubMsg = SubMsg{
            id : INSTANTIATE_TOKEN_ID,
            gas_limit : None,
            reply_on : ReplyOn::Success,
            msg : WasmMsg::Instantiate { 
                admin: None, 
                code_id: NFT_CODE_ID.load(deps.storage)?, 
                msg: to_json_binary(&MintMsg{
                    token_name : token_name.to_owned(),
                    token_symbol : token_symbol.to_owned(),
                    price_per_share,
                    stocks,
                })?, 
                funds: vec![], 
                label: format!("Stock {token_name} with symbol {token_symbol}, price per share {price_per_share} and no. of stocks {stocks}").to_string(),
            }.into(),
        };

        Ok(Response::new()
        .add_submessage(nft_msg)
        .add_attribute("action", "Stock minted")
        .add_attribute("stock name", token_name)
        .add_attribute("symbol", token_symbol)
        .add_attribute("price_per_share", price_per_share)
        .add_attribute("stocks", stocks))
    }

    pub fn buy_stock(deps : DepsMut, info : MessageInfo, token_symbol : String, stock_address : Addr) -> Result<Response, ContractError>{
        let mut unbought_stock_info = UNBOUGHT_STOCKS.load(deps.storage)?;
        
        let sym_pos = unbought_stock_info.iter().position(|x| (*x).symbol == token_symbol);

        match sym_pos {
            None => Err(ContractError::StockNotFound),
            Some(sym_pos) => {
                
                let pos = unbought_stock_info[sym_pos].stocks.iter().position(|x| *x == stock_address);

                match pos {
                    None => Err(ContractError::StockNotFound),
                    Some(pos) => {
                        unbought_stock_info[sym_pos].stocks.remove(pos);
                        UNBOUGHT_STOCKS.save(deps.storage, &unbought_stock_info)?;

                        let bought_stocks_info = BOUGHT_STOCKS.may_load(deps.storage, info.sender.to_owned())?;

                        match bought_stocks_info {
                            None => {
                                let stock_cat_vec = vec![StockCategory{
                                    symbol : token_symbol.to_owned(),
                                    stocks : vec![stock_address.to_owned()]
                                }];

                                BOUGHT_STOCKS.save(deps.storage, info.sender.to_owned(), &stock_cat_vec)?;

                                Ok(Response::new()
                                .add_attribute("action", "buy_stock")
                                .add_attribute("buyer", info.sender)
                                .add_attribute("symbol", token_symbol)
                                .add_attribute("stock_address", stock_address))
                            },
                            Some(mut bought_stocks_info) => {
                                let bought_pos = bought_stocks_info.iter().position(|x| (*x).symbol == token_symbol );

                        match bought_pos {
                            None => {
                                let stock_cat = StockCategory{
                                    symbol : token_symbol.to_owned(),
                                    stocks : vec![stock_address.to_owned()],
                                };

                                bought_stocks_info.push(stock_cat);
                            },
                            Some(bought_pos) => {
                                bought_stocks_info[bought_pos].stocks.push(stock_address.to_owned());
                            }
                        }

                        BOUGHT_STOCKS.save(deps.storage, info.sender.to_owned(), &bought_stocks_info)?;

                        Ok(Response::new()
                        .add_attribute("action", "buy_stock")
                        .add_attribute("buyer", info.sender)
                        .add_attribute("symbol", token_symbol)
                        .add_attribute("stock_address", stock_address))
                    }
                }
            }
                            }
                        }

                        
        }
    }

    pub fn sell_stock(deps : DepsMut, info: MessageInfo, token_symbol : String, stock_address : Addr) -> Result<Response, ContractError> {
        let sender_stocks_info = BOUGHT_STOCKS.may_load(deps.storage, info.sender.to_owned())?;

        match sender_stocks_info {
            None => Err(ContractError::StockNotFound),
            Some(mut sender_stocks_info) => {
                let sym_pos = sender_stocks_info.iter().position(|x| (*x).symbol == token_symbol.to_owned());

        match sym_pos{
            None => Err(ContractError::StockNotFound),
            Some(sym_pos) => {
                let pos = sender_stocks_info[sym_pos].stocks.iter().position(|x| *x == stock_address.to_owned() );

                match pos{
                    None => Err(ContractError::StockNotFound),
                    Some(pos) => {
                        sender_stocks_info[sym_pos].stocks.remove(pos);

                        let mut unbought_stocks_info = UNBOUGHT_STOCKS.load(deps.storage)?;

                        let unbought_pos = unbought_stocks_info.iter().position(|x| (*x).symbol == token_symbol.to_owned());

                        match unbought_pos{
                            None => {
                                let stock_cat = StockCategory{
                                    symbol : token_symbol.to_owned(),
                                    stocks : vec![stock_address.to_owned()],
                                };

                                unbought_stocks_info.push(stock_cat);
                            },
                            Some(unbought_pos) => {
                                unbought_stocks_info[unbought_pos].stocks.push(stock_address.to_owned());
                            }
                        }

                        UNBOUGHT_STOCKS.save(deps.storage, &unbought_stocks_info)?;

                        Ok(Response::new()
                        .add_attribute("action", "sell_stock")
                        .add_attribute("seller", info.sender)
                        .add_attribute("token_symbol", token_symbol)
                        .add_attribute("stock_address", stock_address))
                    }
                }
            }
    }
            }
        }

        
}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let reply = parse_reply_instantiate_data(msg.to_owned()).unwrap();

    let contract_address = Addr::unchecked(reply.contract_address);

    let mut unbought_stocks_info = UNBOUGHT_STOCKS.load(deps.storage)?;

    let events = msg.result.into_result().unwrap().events;
    let wasm_events = events.iter().find(|x| x.ty == "wasm".to_string());

    match wasm_events {
        None => Err(ContractError::NoField),
        Some(wasm_events) => {
            let token_symbol_attri = wasm_events.attributes.iter().find(|x| x.key == "token_symbol".to_string());

            match token_symbol_attri {
                None => Err(ContractError::NoField),
                Some(token_symbol_attri) => {
                    let token_symbol = token_symbol_attri.value.to_owned();

                    let sym_pos = unbought_stocks_info.iter().position(|x| (*x).symbol == token_symbol.to_owned());

                match sym_pos {
                    None => {
                        let stock_cat = StockCategory{
                            symbol : token_symbol.to_owned(),
                            stocks : vec![contract_address.to_owned()],
                        };

                        unbought_stocks_info.push(stock_cat);
                    },
                    Some(sym_pos) => {
                        unbought_stocks_info[sym_pos].stocks.push(contract_address.to_owned());
                    }
                }

                UNBOUGHT_STOCKS.save(deps.storage, &unbought_stocks_info)?;

                Ok(Response::new()
                .add_attribute("action", "Stored new stock NFT")
                .add_attribute("token_symbol", token_symbol)
                .add_attribute("stock_address", contract_address))
                            }
                        }
                    }
                }

    
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BoughtStocks { address } => to_json_binary(&query::bought_stocks(deps, address)?),
        QueryMsg::UnboughtStocks {  } => to_json_binary(&query::unbought_stocks(deps)?),
        QueryMsg::QueryDetails { stock_address } => to_json_binary(&query::query_details(deps, stock_address)?),
    }
}

pub mod query {
    use cosmwasm_std::QueryRequest;

    use crate::msg::{StockInfo, SymbolStock};

    use super::*;

    #[cw_serde]
    pub struct NFTDetails{
        pub get_details : GetDetails,
    }

    #[cw_serde]
    pub struct GetDetails{}

    pub fn bought_stocks(deps : Deps, address : Addr) -> StdResult<AllStocksResponse> {
        let bought_stocks_info = BOUGHT_STOCKS.load(deps.storage, address)?;

        let mut stock_details : Vec<SymbolStock> = vec![];

        for stock_cat in bought_stocks_info.iter() {
            let mut sym_stocks = SymbolStock{
                symbol : stock_cat.symbol.to_owned(),
                stock_info : vec![],
            };
            for stock_address in stock_cat.stocks.iter() {
                let temp = StockInfo{
                    address : stock_address.to_owned(),
                    contract_info : query_details(deps, stock_address.to_owned())?,
                };

                sym_stocks.stock_info.push(temp);
            }

            stock_details.push(sym_stocks);
        }
        let stock_response = AllStocksResponse{
            stocks : stock_details,
        };

        Ok(stock_response)
    }

    pub fn unbought_stocks(deps : Deps) -> StdResult<AllStocksResponse> {
        let unbought_stocks_info = UNBOUGHT_STOCKS.load(deps.storage)?;

        let mut stock_details : Vec<SymbolStock> = vec![];

        for stock_cat in unbought_stocks_info.iter() {
            let mut sym_stocks = SymbolStock{
                symbol : stock_cat.symbol.to_owned(),
                stock_info : vec![],
            };
            for stock_address in stock_cat.stocks.iter() {
                let temp = StockInfo{
                    address : stock_address.to_owned(),
                    contract_info : query_details(deps, stock_address.to_owned())?,
                };

                sym_stocks.stock_info.push(temp);
            }

            stock_details.push(sym_stocks);
        }
        let stock_response = AllStocksResponse{
            stocks : stock_details,
        };

        Ok(stock_response)
    }

    pub fn query_details(deps : Deps, address : Addr) -> StdResult<ContractInfo>{
        let details_request = QueryRequest::Wasm(
            WasmQuery::Smart {
                contract_addr : address.to_string(),
                msg : to_json_binary(&NFTDetails{ get_details : GetDetails {}})?,
            }
        );

        let stock_detail : ContractInfo = deps.querier.query(&details_request)?;

        Ok(stock_detail)
    }
}


#[cfg(test)]
mod tests {
    
}