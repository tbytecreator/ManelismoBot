use actix_web::{web, HttpResponse};
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
    let generated = bot.generate_response(question).await;

    info!("Generated response for question");

    HttpResponse::Ok().json(AnswerResponse {
        answer: generated.answer,
        confidence: generated.confidence,
        source: "ManelismoBot v0.1".to_string(),
    })
}
