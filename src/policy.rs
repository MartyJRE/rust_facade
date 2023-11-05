use crate::definition::Policy;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct SetEnvironment {
    description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BetterInvoke {
    #[serde(rename = "target-url")]
    target_url: String,
    timeout: u16,
    verb: String,
    #[serde(rename = "input-body")]
    input_body: Option<String>,
    forever: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseHandler {
    #[serde(rename = "clear-body")]
    clear_body: Option<bool>,
    #[serde(rename = "stjs-data-holder")]
    stjs_data_holder: Option<String>,
    #[serde(rename = "success-code")]
    success_code: Option<u16>,
    #[serde(rename = "hard-fail")]
    hard_fail: Option<bool>,
    #[serde(rename = "set-context")]
    set_context: Option<HashMap<String, String>>,
    #[serde(rename = "set-headers")]
    set_headers: Option<HashMap<String, String>>,
    frontend: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Operation {
    pub path: String,
    pub verb: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OperationSwitchCase {
    pub operations: Vec<Operation>,
    pub execute: Vec<Policy>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OperationSwitch {
    pub title: String,
    #[serde(rename = "case")]
    pub cases: Vec<OperationSwitchCase>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorMessageHandling {
    description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Javascript {
    title: String,
    source: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct If {
    condition: String,
    execute: Vec<Policy>,
}
