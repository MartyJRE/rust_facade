use serde::Deserialize;
use std::collections::HashMap;
use std::fs::read_to_string;

use serde_yaml::Value;
use std::process::exit;
use walkdir::WalkDir;

fn main() {
    let definitions = parse_definitions("definitions");
    for def in definitions {
        println!("{def:?}");
    }
}

fn parse_definitions(directory: &str) -> Vec<Definition> {
    let mut definitions = vec![];
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        match entry.path().to_str() {
            Some(path) => {
                let contents = read_to_string(path);
                match contents {
                    Ok(content) => {
                        let definition = serde_yaml::from_str(content.as_str());
                        match definition {
                            Ok(definition) => {
                                definitions.push(definition);
                            }
                            Err(error) => {
                                eprintln!("Could not parse {path}: {error}");
                                exit(1);
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("Could not read the contents of {path}");
                        exit(1);
                    }
                }
            }
            None => {
                eprintln!("Could not process one of the files in {directory}");
                exit(1);
            }
        }
    }
    return definitions;
}

#[derive(Deserialize, Debug)]
struct Info {
    version: String,
    title: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct Parameter {
    #[serde(rename = "in")]
    location: String,
    name: String,
    description: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SubError {
    code: u16,
    message: String,
}

#[derive(Deserialize, Debug)]
struct ContentSchema {
    path: String,
    property: String,
    message: Option<String>,
    code: Option<u16>,
    #[serde(rename = "type")]
    content_type: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ErrorCode {
    Integer(u16),
    Float(f32),
    String(String),
}

#[derive(Deserialize, Debug)]
struct ErrorSchema {
    code: ErrorCode,
    message: String,
    #[serde(rename = "suberrors")]
    sub_errors: Option<Vec<SubError>>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ResponseSchema {
    Content {
        #[serde(rename = "x-js-schema", alias = "x-js-content", alias = "x-js-type")]
        schema: ContentSchema,
    },
    Error {
        #[serde(rename = "x-js-error-content", alias = "x-error-message")]
        schema: ErrorSchema,
    },
}

#[derive(Deserialize, Debug)]
struct Response {
    description: String,
    #[serde(flatten)]
    schemas: Option<Vec<ResponseSchema>>,
}

#[derive(Deserialize, Debug)]
struct Method {
    parameters: Option<Vec<Parameter>>,
    summary: Option<String>,
    description: Option<String>,
    responses: Option<HashMap<u16, Response>>,
}

#[derive(Deserialize, Debug)]
struct SetEnvironment {
    description: String,
}

#[derive(Deserialize, Debug)]
struct BetterInvoke {
    #[serde(rename = "target-url")]
    target_url: String,
    timeout: u16,
    verb: String,
    #[serde(rename = "input-body")]
    input_body: String,
    forever: bool,
}

#[derive(Deserialize, Debug)]
struct ResponseHandler {
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

#[derive(Deserialize, Debug)]
struct Operation {
    path: String,
    verb: String,
}

#[derive(Deserialize, Debug)]
struct OperationSwitchCase {
    operations: Vec<Operation>,
    execute: Vec<Policy>,
}

#[derive(Deserialize, Debug)]
struct OperationSwitch {
    title: String,
    #[serde(rename = "case")]
    cases: Vec<OperationSwitchCase>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Policy {
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
    Other(HashMap<String, Value>),
}

#[derive(Deserialize, Debug)]
struct Assembly {
    #[serde(rename = "execute")]
    policies: Vec<Policy>,
}

#[derive(Deserialize, Debug)]
struct Switch {
    assembly: Assembly,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Definition {
    info: Info,
    base_path: String,
    swagger: String,
    consumes: Vec<String>,
    produces: Vec<String>,
    paths: HashMap<String, HashMap<String, Method>>,
    #[serde(rename = "x-ibm-configuration")]
    switch: Switch,
}
