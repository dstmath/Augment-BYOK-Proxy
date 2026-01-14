#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct OpenAIStreamOptions {
  pub include_usage: bool,
}

#[derive(Debug, Serialize)]
pub struct OpenAIChatCompletionRequest {
  pub model: String,
  pub messages: Vec<OpenAIChatMessage>,
  pub stream: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stream_options: Option<OpenAIStreamOptions>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_tokens: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub temperature: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub top_p: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tools: Option<Vec<OpenAITool>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_choice: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct OpenAIChatMessage {
  pub role: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_calls: Option<Vec<OpenAIToolCall>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_call_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OpenAITool {
  #[serde(rename = "type")]
  pub tool_type: String,
  pub function: OpenAIFunctionDefinition,
}

#[derive(Debug, Serialize)]
pub struct OpenAIFunctionDefinition {
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIToolCall {
  pub id: String,
  #[serde(rename = "type")]
  pub call_type: String,
  pub function: OpenAIFunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIFunctionCall {
  pub name: String,
  pub arguments: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatCompletionChunk {
  #[serde(default)]
  pub choices: Vec<OpenAIChoice>,
  #[serde(default)]
  pub usage: Option<OpenAIUsage>,
}

#[derive(Debug, Default, Deserialize)]
pub struct OpenAIChoice {
  #[serde(default)]
  pub delta: OpenAIDelta,
  #[serde(default)]
  pub finish_reason: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct OpenAIDelta {
  #[serde(default)]
  pub role: Option<String>,
  #[serde(default)]
  pub content: Option<String>,
  #[serde(default)]
  pub tool_calls: Option<Vec<OpenAIDeltaToolCall>>,
  #[serde(default)]
  pub function_call: Option<OpenAIDeltaFunctionCall>,
}

#[derive(Debug, Default, Deserialize)]
pub struct OpenAIDeltaToolCall {
  #[serde(default)]
  pub index: Option<usize>,
  #[serde(default)]
  pub id: Option<String>,
  #[serde(default, rename = "type")]
  pub call_type: Option<String>,
  #[serde(default)]
  pub function: Option<OpenAIDeltaFunction>,
}

#[derive(Debug, Default, Deserialize)]
pub struct OpenAIDeltaFunction {
  #[serde(default)]
  pub name: Option<String>,
  #[serde(default)]
  pub arguments: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct OpenAIDeltaFunctionCall {
  #[serde(default)]
  pub name: Option<String>,
  #[serde(default)]
  pub arguments: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct OpenAIUsage {
  #[serde(default)]
  pub prompt_tokens: Option<i64>,
  #[serde(default)]
  pub completion_tokens: Option<i64>,
  #[serde(default)]
  pub total_tokens: Option<i64>,
}
