use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use cw_starter::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, AllStocksResponse, SymbolStock, StockInfo, ContractInfo};
use cw_starter::state::{Config, StockCategory};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    // Previous code omitted
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(StockCategory), &out_dir);
    export_schema(&schema_for!(AllStocksResponse), &out_dir);
    export_schema(&schema_for!(SymbolStock), &out_dir);
    export_schema(&schema_for!(StockInfo), &out_dir);
    export_schema(&schema_for!(ContractInfo), &out_dir);
}
