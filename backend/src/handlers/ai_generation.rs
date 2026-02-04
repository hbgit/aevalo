// AI Generation Handler
// Generates evaluation items using Gemini API

use axum::{
    extract::{State},
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::ScaleType;

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub description: String,
    pub scale_type: Option<ScaleType>,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub items: Vec<GeneratedItem>,
    pub scale_type: ScaleType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneratedItem {
    pub order: i32,
    pub text: String,
    pub metadata: serde_json::Value,
}

/// POST /evaluations/generate
/// Generate evaluation items using AI
pub async fn generate_items_ai(
    State(_db): State<PgPool>,
    Json(req): Json<GenerateRequest>,
) -> Result<(StatusCode, Json<GenerateResponse>), AppError> {
    // Determine scale type
    let scale_type = req.scale_type.unwrap_or(ScaleType::Likert);

    // Call Gemini API to generate items
    let items = call_gemini_api(&req.description, &scale_type).await?;

    Ok((StatusCode::OK, Json(GenerateResponse {
        items,
        scale_type,
    })))
}

/// Call Gemini API to generate evaluation items
async fn call_gemini_api(
    description: &str,
    scale_type: &ScaleType,
) -> Result<Vec<GeneratedItem>, AppError> {
    let api_key = std::env::var("GEMINI_API_KEY")
        .map_err(|_| AppError::ValidationError("GEMINI_API_KEY not set".to_string()))?;

    let prompt = build_prompt(description, scale_type);

    let client = reqwest::Client::new();
    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent")
        .header("Content-Type", "application/json")
        .query(&[("key", api_key.as_str())])
        .json(&serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }]
        }))
        .send()
        .await
        .map_err(|e| AppError::InternalServerError {
            message: format!("Gemini API error: {}", e),
        })?;

    if !response.status().is_success() {
        return Err(AppError::InternalServerError {
            message: "Gemini API request failed".to_string(),
        });
    }

    let response_body: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::InternalServerError {
            message: format!("Failed to parse Gemini response: {}", e),
        })?;

    // Parse response and extract items
    let items = parse_gemini_response(&response_body, scale_type)?;

    Ok(items)
}

/// Build structured prompt for Gemini API
fn build_prompt(description: &str, scale_type: &ScaleType) -> String {
    let scale_instructions = match scale_type {
        ScaleType::Likert => {
            "Scale: Likert 1-5 (1=Strongly Disagree, 5=Strongly Agree)"
        }
        ScaleType::Frequency => {
            "Scale: Frequency (Never, Rarely, Sometimes, Often, Always)"
        }
        ScaleType::PairedComparison => {
            "Scale: Paired Comparison (Compare items pairwise)"
        }
        ScaleType::FixedSum => {
            "Scale: Fixed Sum (Allocate 100 points across items)"
        }
    };

    format!(
        r#"
        Generate 5-10 evaluation survey questions for the following purpose:
        {description}

        {scale_instructions}

        Return ONLY a JSON array with no markdown formatting. Each item must have:
        - order (number 1-10)
        - text (question text)
        - metadata (object with scale-specific settings)

        Format:
        [
          {{
            "order": 1,
            "text": "Question text here",
            "metadata": {{}}
          }}
        ]
        
        Respond ONLY with valid JSON array, no other text.
        "#
    )
}

/// Parse Gemini API response
fn parse_gemini_response(
    response: &serde_json::Value,
    scale_type: &ScaleType,
) -> Result<Vec<GeneratedItem>, AppError> {
    // Extract text from Gemini response
    let text = response
        .get("candidates")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("content"))
        .and_then(|c| c.get("parts"))
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("text"))
        .and_then(|t| t.as_str())
        .ok_or(AppError::InternalServerError {
            message: "Invalid Gemini response format".to_string(),
        })?;

    // Parse JSON from text
    let items: Vec<GeneratedItem> = serde_json::from_str(text)
        .map_err(|e| AppError::InternalServerError {
            message: format!("Failed to parse generated items: {}", e),
        })?;

    // Ensure items have proper metadata
    let items = items
        .into_iter()
        .map(|mut item| {
            // Add scale-specific metadata if missing
            if item.metadata.is_null() || item.metadata.as_object().map(|o| o.is_empty()).unwrap_or(false) {
                item.metadata = match scale_type {
                    ScaleType::Likert => serde_json::json!({
                        "min_value": 1,
                        "max_value": 5,
                        "labels": ["Strongly Disagree", "Disagree", "Neutral", "Agree", "Strongly Agree"]
                    }),
                    ScaleType::Frequency => serde_json::json!({
                        "categories": ["Never", "Rarely", "Sometimes", "Often", "Always"]
                    }),
                    _ => serde_json::json!({}),
                };
            }
            item
        })
        .collect();

    Ok(items)
}

/// POST /evaluations/validate
/// Validate evaluation structure and items
#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    pub items: Vec<GeneratedItem>,
    pub scale_type: ScaleType,
}

#[derive(Debug, Serialize)]
pub struct ValidateResponse {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

pub async fn validate_items(
    Json(req): Json<ValidateRequest>,
) -> Result<(StatusCode, Json<ValidateResponse>), AppError> {
    let mut errors = vec![];

    // Check minimum items
    if req.items.is_empty() {
        errors.push("At least 1 item required".to_string());
    }

    // Validate based on scale type
    match req.scale_type {
        ScaleType::Likert | ScaleType::Frequency => {
            // Standard validation
            if req.items.len() > 50 {
                errors.push("Too many items (max 50)".to_string());
            }
        }
        ScaleType::FixedSum => {
            // Validate fixed sum structure
            for item in &req.items {
                if item.text.is_empty() {
                    errors.push("Empty item text".to_string());
                    break;
                }
            }
        }
        ScaleType::PairedComparison => {
            // Validate at least 3 items for paired comparison
            if req.items.len() < 3 {
                errors.push("Paired comparison requires at least 3 items".to_string());
            }
        }
    }

    // Validate order sequence
    let orders: Vec<i32> = req.items.iter().map(|i| i.order).collect();
    if orders.len() != orders.iter().collect::<std::collections::HashSet<_>>().len() {
        errors.push("Duplicate item orders".to_string());
    }

    Ok((StatusCode::OK, Json(ValidateResponse {
        is_valid: errors.is_empty(),
        errors,
    })))
}
