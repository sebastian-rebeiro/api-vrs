use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub struct methodResponse {
    params: params,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
struct params {
    param: param,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
struct param {
    value: value,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
struct value {
    #[serde(rename = "struct")]
    structure: structure,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
struct structure {
    member: Vec<member>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
struct member {
    name: String,
    value: valuef,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
struct valuef {
    string: String,
}
