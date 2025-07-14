use ::reqwest;
use ::serde_json;
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use serde::{Deserialize, Serialize};
use zigfrid::{Result, lm_models};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to Ollama
    let ollama = Ollama::default();
    let model = format!(
        "{}:{}",
        lm_models::GEMMA_3_MODEL.name,
        lm_models::GEMMA_3_MODEL.size
    );

    // Submit initial verification request
    let verification_prompt: Verification = Verification::new().await;
    println!("{:?}", verification_prompt.response);

    // Generate a response using Ollama hosted model
    let gen_req = GenerationRequest::new(model, verification_prompt.response.text);
    let res = ollama.generate(gen_req).await?;

    println!("->> {}", res.response);

    Ok(())
}

#[derive(Debug)]
pub struct Verification {
    pub response: VerificationResponse,
}

impl Verification {
    async fn new() -> Self {
        Verification {
            response: reqwest::Client::new()
                .post("https://xyz.ag3nts.org/verify")
                .body(VerificationRequest::new("READY".to_string(), 0).to_json())
                .send()
                .await
                .expect("Failed get response from URL.")
                .text()
                .await
                .expect("Failed to parse the response")
                .into(),
        }
    }
}

pub struct VerificationError {
    pub output: Option<i16>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationResponse {
    pub text: String,
    #[serde(rename = "msgID")]
    pub msg_id: i32,
}

impl VerificationResponse {
    pub fn new(text: String, msg_id: i32) -> Self {
        VerificationResponse { text, msg_id }
    }
}

impl From<String> for VerificationResponse {
    fn from(response: String) -> Self {
        serde_json::from_str(&response).expect("Failed to parse VerificationResponse from string")
    }
}

// impl GPTizable for VerificationResponse {
//     fn conversation(&self) -> &str {
//         "Verification Conversation"
//     }

//     fn send_message(&self, message: &str) -> Result<String, VerificationError> {}
// }

trait GPTizable {
    /// Trait that ensures that a struct contains a `conversation` field and implements a method to send the message to the Ollama API.
    fn conversation(&self) -> &str;
    fn send_message(&self, message: &str) -> core::result::Result<String, VerificationError>;
}

#[derive(Serialize, Deserialize)]
pub struct VerificationRequest {
    pub text: String,
    #[serde(rename = "msgID")]
    pub msg_id: i32,
}

impl VerificationRequest {
    pub fn new(text: String, msg_id: i32) -> Self {
        VerificationRequest { text, msg_id }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize VerificationRequest to JSON")
    }
}
