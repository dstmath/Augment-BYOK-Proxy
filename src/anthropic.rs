#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AnthropicRequest {
  pub model: String,
  pub messages: Vec<AnthropicMessage>,
  pub max_tokens: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub temperature: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub top_p: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub top_k: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_sequences: Option<Vec<String>>,
  pub stream: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tools: Option<Vec<AnthropicTool>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_choice: Option<AnthropicToolChoice>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub thinking: Option<AnthropicThinking>,
}

#[derive(Debug, Serialize)]
pub struct AnthropicThinking {
  #[serde(rename = "type")]
  pub thinking_type: String,
  pub budget_tokens: u32,
}

#[derive(Debug, Serialize)]
pub struct AnthropicMessage {
  pub role: String,
  pub content: Vec<AnthropicContentBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicContentBlock {
  #[serde(rename = "type")]
  pub block_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<AnthropicImageSource>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub input: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_use_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_error: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub thinking: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicImageSource {
  #[serde(rename = "type")]
  pub source_type: String,
  pub media_type: String,
  pub data: String,
}

#[derive(Debug, Serialize)]
pub struct AnthropicTool {
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  pub input_schema: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct AnthropicToolChoice {
  #[serde(rename = "type")]
  pub choice_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AnthropicStreamEvent {
  #[serde(default, rename = "type")]
  pub event_type: String,
  #[serde(default)]
  pub index: Option<i64>,
  #[serde(default)]
  pub message: Option<AnthropicResponse>,
  #[serde(default)]
  pub content_block: Option<AnthropicContentBlock>,
  #[serde(default)]
  pub delta: Option<AnthropicDelta>,
  #[serde(default)]
  pub usage: Option<AnthropicUsage>,
}

#[derive(Debug, Deserialize)]
pub struct AnthropicDelta {
  #[serde(default, rename = "type")]
  pub delta_type: String,
  #[serde(default)]
  pub text: Option<String>,
  #[serde(default)]
  pub partial_json: Option<String>,
  #[serde(default)]
  pub thinking: Option<String>,
  #[serde(default)]
  pub stop_reason: Option<String>,
  #[serde(default)]
  pub stop_sequence: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AnthropicResponse {
  #[serde(default)]
  pub id: String,
  #[serde(default, rename = "type")]
  pub resp_type: String,
  #[serde(default)]
  pub role: String,
  #[serde(default)]
  pub content: Vec<AnthropicContentBlock>,
  #[serde(default)]
  pub model: String,
  #[serde(default)]
  pub stop_reason: String,
  #[serde(default)]
  pub stop_sequence: Option<String>,
  #[serde(default)]
  pub usage: AnthropicUsage,
}

#[derive(Debug, Default, Deserialize)]
pub struct AnthropicUsage {
  #[serde(default)]
  pub input_tokens: Option<i64>,
  #[serde(default)]
  pub output_tokens: Option<i64>,
  #[serde(default)]
  pub cache_read_input_tokens: Option<i64>,
  #[serde(default)]
  pub cache_creation_input_tokens: Option<i64>,
}
