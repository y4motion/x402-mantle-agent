use serde::Serialize;

#[derive(Serialize)]
pub struct IpfsPayload<'a> {
    pub version: &'a str,
    pub payload: &'a str,
}

pub fn prepare_ipfs_payload(data: &str) -> String {
    let payload = IpfsPayload {
        version: "1.0",
        payload: data,
    };
    serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string())
}
