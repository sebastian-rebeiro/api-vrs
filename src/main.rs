#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use reqwest::Response;
// use serde::{Deserialize, Serialize};
// use serde_xml_rs::{from_str, to_string};

async fn simple_post(url: String, body: String) -> Response {
    let response = reqwest::Client::new()
        .post(url)
        .body(body)
        .send()
        .await
        .expect("Send Unssucsessful");
    response
}

// #[derive(, Serialize, Deserialize, PartialEq)]

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/client/<clientnum>")]
async fn hello(clientnum: u32) -> String {
    let uri: String = String::from("http://billing.dido.ca:8008");

    let front: String = String::from("<methodCall><methodName>FS.API.customer_info</methodName><params><param><value><string>secret</string></value></param><param><value><string></string></value></param><param><value><string>custnum</string></value></param><param><value><string>");
    let back: String = String::from("</string></value></param></params></methodCall>");

    let post = format!("{front}{clientnum}{back}");

    simple_post(uri, post)
        .await
        .text()
        .await
        .expect("error reading")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1/", routes![index, hello])
}
