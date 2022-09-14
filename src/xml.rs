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
pub struct param {
    value: value,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub struct value {
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

#[derive(Debug)]
struct customer {
    country: String,
    status: String,
    invoicing_list: String,
    state: String,
    ship_state: String,
    ship_city: String,
    city: String,
    name: String,
    first: String,
    last: String,
    error: String,
    salesnum: String,
    balance: f32,
    ship_address1: String,
    ship_address2: String,
    ship_country: String,
    address1: String,
    ship_zip: String,
    agentnum: String,
    referral_custnum: u32,
    display_custnum: u32,
    county: String,
    refnum: i32,
    ship_county: String,
    fax: String,
    night: String,
    zip: String,
    mobile: String,
    statuscolor: String,
    classnum: i32,
    postal_invoicing: String,
    company: String,
    daytime: String,
    usernum: String,
    address2: String,
}
