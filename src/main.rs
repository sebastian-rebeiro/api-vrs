#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod xml;
use rocket::response::content;

const URI: &str = "http://billing.dido.ca:8008";

async fn simple_post(url: &str, body: String) -> String {
    let request: String = reqwest::Client::new() // Making new request "client"
        .post(url) // Always a post method to XML-RPC server
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

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/client/<clientnum>")]
async fn customer_info(clientnum: u32) -> content::RawJson<String> {
    let front: String = String::from("<methodCall><methodName>FS.API.customer_info</methodName><params><param><value><string>secret</string></value></param><param><value><string></string></value></param><param><value><string>custnum</string></value></param><param><value><string>");
    let back: String = String::from("</string></value></param></params></methodCall>");

    let post: String = format!("{front}{clientnum}{back}");

    content::RawJson(simple_post(&URI, post).await)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1/", routes![index, customer_info])
}
