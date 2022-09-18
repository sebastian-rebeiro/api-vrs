#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod fromreq;
mod xml;
use rocket::{response::content, serde::json::Json};
use serde::Deserialize;

const URI: &str = "http://billing.dido.ca:8008";

async fn simple_post(url: &str, body: String) -> String {
    let request: String = reqwest::Client::new() // Making new request "client"
        .post(url) //
        .body(body)
        .send() // Sending
        .await
        .expect("Send Unssucsessful")
        .text() // Raw XML data
        .await
        .expect("error reading");

    let request = request
        .replace("<methodResponse>", "")
        .replace("</methodResponse>", "")
        .replace("<params>", "")
        .replace("</params>", "")
        .replace("<param>", "")
        .replace("</param>", "")
        .replace("<struct>", "")
        .replace("</struct>", "");

    let deserialized: xml::structure = serde_xml_rs::from_str(&request).unwrap();
    serde_json::to_string(&deserialized).unwrap()
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Methods {
    customer_info,
    insert_payment,
    insert_refund,
    new_customer,
    update_customer,
    location_info,
    bill_now,
}

#[derive(Debug)]
struct Params {
    name: Param,
    value: String,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Param {
    secret,
    custnum,
    payby,
    paid,
    _date,
    amount,
    first,
    last,
    ss,
    company,
    address1,
    city,
    county,
    state,
    zip,
    country,
    latitude,
    longitude,
    geocode,
    censustract,
    censusyear,
    daytime,
    night,
    fax,
    mobile,
    invoicing_list,
    payinfo,
    paycvv,
    paydate,
    payname,
    referral_custnum,
    salesnum,
    agentnum,
    agent_custid,
    refferal_custnum,
}

#[derive(Debug, Deserialize)]
struct inp_customer {
    first: String,
    last: String,
    ss: String,
    company: String,
    address1: String,
    city: String,
    county: String,
    state: String,
    zip: String,
    country: String,
    latituge: String,
    longitude: String,
    geocode: String,
    censustract: String,
    censusyear: String,
    daytime: String,
    night: String,
    fax: String,
    mobile: String,
    invoicing_list: String,
    payby: String,
    payinfo: String,
    paycvv: String,
    paydate: String,
    payname: String,
    referral_custnum: String,
    salesnum: String,
    agentnum: String,
    agent_custid: String,
}

fn outgoing_body(method_name: Methods, params: Vec<Params>) -> String {
    let mut outgoing = String::new();

    for param in params {
        let x = format!("<param><value><string>{:?}</string></value></param><param><value><string>{}</string></value></param>", param.name, param.value);
        outgoing = [outgoing, x].join("");
    }

    format!(
        "<methodCall><methodName>FS.API.{:?}</methodName><params>{}</params></methodCall>",
        method_name, outgoing
    )
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/client/<clientnum>", format = "application/json")]
async fn customer_info(clientnum: u32, apisecret: fromreq::ApiKey) -> content::RawJson<String> {
    let post: String = outgoing_body(
        Methods::customer_info,
        vec![
            Params {
                name: Param::secret,
                value: apisecret.0,
            },
            Params {
                name: Param::custnum,
                value: clientnum.to_string(),
            },
        ],
    );
    // let post: String = format!("<methodCall><methodName>FS.API.customer_info</methodName><params><param><value><string>secret</string></value></param><param><value><string>{}</string></value></param><param><value><string>custnum</string></value></param><param><value><string>{}</string></value></param></params></methodCall>", "", clientnum);

    content::RawJson(simple_post(&URI, post).await)
}

#[get("/sensitive")]
fn sensitive(key: fromreq::ApiKey) -> String {
    format!("this key is valid {}", key.0)
}

#[route(POST, uri = "/client", data = "<data>")]
async fn new_customer(data: Json<String>, apisecret: fromreq::ApiKey) -> content::RawJson<String> {
    println!("{:?}", data);

    let post: String = outgoing_body(
        Methods::customer_info,
        vec![
            Params {
                name: Param::secret,
                value: apisecret.0,
            },
            Params {
                name: Param::custnum,
                value: String::from(""),
            },
        ],
    );
    content::RawJson(simple_post(&URI, post).await)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api/v1/",
        routes![index, new_customer, customer_info, sensitive],
    )
}
// https://youtu.be/2RWXeosWhAQ?t=886
