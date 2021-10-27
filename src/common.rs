use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Base {
    pub denom: String,
    pub filled: Uint128,
    pub size: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Quote {
    pub denom: String,
    pub filled: Uint128,
    pub size: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Fee {
    pub denom: String,
    pub filled: Uint128,
    pub size: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FeeInfo {
    pub account: Addr,
    pub rate: String,
}