use crate::policy::{
    BetterInvoke, ErrorMessageHandling, If, Javascript, ResponseHandler, SetEnvironment,
};
use serde::Deserialize;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Info {
    pub version: String,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Parameter {
    #[serde(rename = "in")]
    pub location: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SubError {
    pub code: u16,
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContentSchema {
    pub path: String,
    pub property: String,
    pub message: Option<String>,
    pub code: Option<u16>,
    #[serde(rename = "type")]
    pub content_type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ErrorCode {
    Integer(u16),
    Float(f32),
    String(String),
}

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorSchema {
    pub code: ErrorCode,
    pub message: String,
    #[serde(rename = "suberrors")]
    pub sub_errors: Option<Vec<SubError>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ResponseSchema {
    Content {
        #[serde(rename = "x-js-schema", alias = "x-js-content", alias = "x-js-type")]
        schema: ContentSchema,
    },
    Error {
        #[serde(rename = "x-js-error-content", alias = "x-error-message")]
        schema: ErrorSchema,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    pub description: String,
    #[serde(flatten)]
    pub schemas: Option<Vec<ResponseSchema>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Method {
    pub parameters: Option<Vec<Parameter>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub responses: Option<HashMap<u16, Response>>,
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
#[serde(untagged)]
pub enum Policy {
    SetEnvironment {
        #[serde(rename = "set-environment")]
        policy: SetEnvironment,
    },
    BetterInvoke {
        #[serde(rename = "better-invoke")]
        policy: BetterInvoke,
    },
    ResponseHandler {
        #[serde(rename = "response-handler")]
        policy: ResponseHandler,
    },
    OperationSwitch {
        #[serde(rename = "operation-switch")]
        policy: OperationSwitch,
    },
    ErrorMessageHandling {
        #[serde(rename = "error-message-handling")]
        policy: ErrorMessageHandling,
    },
    Javascript {
        #[serde(rename = "javascript")]
        policy: Javascript,
    },
    If {
        #[serde(rename = "if")]
        policy: If,
    },
    Other(HashMap<String, Value>),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Assembly {
    #[serde(rename = "execute")]
    pub policies: Vec<Policy>,
    pub catch: Vec<Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Switch {
    pub assembly: Assembly,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Definition {
    pub info: Info,
    #[serde(rename = "basePath")]
    pub base_path: String,
    pub swagger: String,
    pub consumes: Vec<String>,
    pub produces: Vec<String>,
    pub paths: HashMap<String, HashMap<String, Method>>,
    #[serde(rename = "x-ibm-configuration")]
    pub switch: Switch,
    pub schemes: Vec<String>,
}
