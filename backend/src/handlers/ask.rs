use actix_web::{web, HttpResponse};
use serde_json::json;
use log::info;
use crate::models::{QuestionRequest, AnswerResponse, ErrorResponse};
use crate::manelismo_bot::ManelismoBot;

pub async fn ask_question(
    body: web::Json<QuestionRequest>,
) -> HttpResponse {
    let question = &body.question;
    
    if question.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid request".to_string(),
            details: "Question cannot be empty".to_string(),
        });
    }

    info!("Received question: {}", question);

    let bot = ManelismoBot::new();
    let answer = bot.generate_response(question);

    info!("Generated response for question");

    HttpResponse::Ok().json(AnswerResponse {
        answer,
        confidence: 0.85,
        source: "ManelismoBot v0.1".to_string(),
    })
}
