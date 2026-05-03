use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestionRequest {
    pub question: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerResponse {
    pub answer: String,
    pub confidence: f32,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub details: String,
}
