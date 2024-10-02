use axum::{http::HeaderMap, response::Html};
use std::fs;

pub async fn policy() -> Html<String> {
    Html(String::from("mailbox-only"))
}

pub async fn pubkey() -> (HeaderMap, Vec<u8>) {
    let mut resp_header = HeaderMap::new();
    resp_header.insert("Content-Type", "application/octet-stream".parse().unwrap());
    resp_header.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    let keyraw = fs::read("assets/files/publickey-binary.asc").unwrap();
    (resp_header, keyraw)
}
