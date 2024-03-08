use axum::http::HeaderMap;

pub async fn did() -> (HeaderMap, String) {
    let mut resp_header = HeaderMap::new();
    resp_header.insert("Content-Type", "text/plain".parse().unwrap());
    resp_header.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    let did = String::from("did:plc:excwqwknn6u3eel2frn42eqm");
    (resp_header, did)
}
