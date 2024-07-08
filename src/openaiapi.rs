use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<i32>,
    pub top_p: Option<f64>,
    pub n: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIResponse {
    pub choices: Vec<OpenAIChoice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIChoice {
    pub message: OpenAIMessage,
}

pub async fn send_request(
    uri: &str,
    api_key: &str,
    openai_request: &OpenAIRequest,
) -> Result<OpenAIResponse, Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);

    let body = Body::from(serde_json::to_vec(&openai_request)?);

    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(body)
        .unwrap();

    let res = client.request(req).await?;

    if !res.status().is_success() {
        let error_message = format!("Request failed with status: {}", res.status());

        let error_body = hyper::body::aggregate(res).await?;
        let plain_json_response = String::from_utf8_lossy(error_body.chunk()).to_string();
        eprintln!("Plain JSON Response: {}", plain_json_response);

        return Err(error_message.into());
    }

    let body = hyper::body::aggregate(res).await?;
    let json: OpenAIResponse = serde_json::from_reader(body.reader())?;
    Ok(json)
}
