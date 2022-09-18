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
}

#[derive(Debug, Deserialize, FromForm)]
struct InpCustomer {
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
    latitude: String,
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

#[route(POST, uri = "/client", data = "<data>")]
async fn new_customer(
    data: Json<InpCustomer>,
    apisecret: fromreq::ApiKey,
) -> content::RawJson<String> {
    println!("{:?}", &data);

    let post: String = outgoing_body(
        Methods::new_customer,
        vec![
            Params {
                name: Param::secret,
                value: apisecret.0,
            },
            Params {
                name: Param::first,
                value: data.0.first,
            },
            Params {
                name: Param::last,
                value: data.0.last,
            },
            Params {
                name: Param::ss,
                value: data.0.ss,
            },
            Params {
                name: Param::company,
                value: data.0.company,
            },
            Params {
                name: Param::address1,
                value: data.0.address1,
            },
            Params {
                name: Param::city,
                value: data.0.city,
            },
            Params {
                name: Param::county,
                value: data.0.county,
            },
            Params {
                name: Param::state,
                value: data.0.state,
            },
            Params {
                name: Param::zip,
                value: data.0.zip,
            },
            Params {
                name: Param::country,
                value: data.0.country,
            },
            Params {
                name: Param::latitude,
                value: data.0.latitude,
            },
            Params {
                name: Param::longitude,
                value: data.0.longitude,
            },
            Params {
                name: Param::geocode,
                value: data.0.geocode,
            },
            Params {
                name: Param::censustract,
                value: data.0.censustract,
            },
            Params {
                name: Param::censusyear,
                value: data.0.censusyear,
            },
            Params {
                name: Param::daytime,
                value: data.0.daytime,
            },
            Params {
                name: Param::night,
                value: data.0.night,
            },
            Params {
                name: Param::fax,
                value: data.0.fax,
            },
            Params {
                name: Param::mobile,
                value: data.0.mobile,
            },
            Params {
                name: Param::invoicing_list,
                value: data.0.invoicing_list,
            },
            Params {
                name: Param::payby,
                value: data.0.payby,
            },
            Params {
                name: Param::payinfo,
                value: data.0.payinfo,
            },
            Params {
                name: Param::paycvv,
                value: data.0.paycvv,
            },
            Params {
                name: Param::paydate,
                value: data.0.paydate,
            },
            Params {
                name: Param::payname,
                value: data.0.payname,
            },
            Params {
                name: Param::referral_custnum,
                value: data.0.referral_custnum,
            },
            Params {
                name: Param::salesnum,
                value: data.0.salesnum,
            },
            Params {
                name: Param::agentnum,
                value: data.0.agentnum,
            },
            Params {
                name: Param::agent_custid,
                value: data.0.agent_custid,
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
