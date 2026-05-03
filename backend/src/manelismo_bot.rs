use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

pub struct ManelismoBot {
    client: Client,
    base_url: String,
    model: String,
    system_prompt: String,
}

pub struct GeneratedAnswer {
    pub answer: String,
    pub confidence: f32,
}

#[derive(Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}

impl ManelismoBot {
    pub fn new() -> Self {
        let base_url = env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".to_string());
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "tinyllama".to_string());

        ManelismoBot {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(90))
                .build()
                .unwrap_or_else(|_| Client::new()),
            base_url,
            model,
            system_prompt: String::from(
                "Você é o Bot do Manelismo, uma persona inspirada em Manoel Alves Ferreira Neto (TByteCreator/Tremyen). \
                 Responda em português do Brasil, de forma direta, crítica e conectada a tecnologia/sociedade. \
                 Evite lero-lero, responda exatamente ao que foi perguntado e use no máximo 6 frases."
            ),
        }
    }

    pub async fn generate_response(&self, question: &str) -> GeneratedAnswer {
        let prompt = format!(
            "Pergunta do usuário: {}\n\nResponda com foco objetivo na pergunta. Se houver ambiguidade, assuma o cenário mais comum e explicite em 1 frase.",
            question.trim()
        );

        match self.generate_with_ollama(&prompt).await {
            Ok(answer) if !answer.trim().is_empty() => GeneratedAnswer {
                answer: answer.trim().to_string(),
                confidence: 0.88,
            },
            _ => GeneratedAnswer {
                answer: fallback_response(question),
                confidence: 0.55,
            },
        }
    }

    async fn generate_with_ollama(&self, prompt: &str) -> Result<String, String> {
        let url = format!("{}/api/generate", self.base_url.trim_end_matches('/'));
        let payload = OllamaGenerateRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            system: self.system_prompt.clone(),
            stream: false,
        };

        let response = self
            .client
            .post(url)
            .json(&payload)
            .send()
            .await
            .map_err(|err| format!("request_error: {}", err))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("ollama_http_{}: {}", status, body));
        }

        let parsed: OllamaGenerateResponse = response
            .json()
            .await
            .map_err(|err| format!("decode_error: {}", err))?;

        Ok(parsed.response)
    }

}

fn fallback_response(question: &str) -> String {
    format!(
        "Ainda estou inicializando o modelo de IA local. Pergunta recebida: \"{}\". \
         Tente novamente em alguns segundos.",
        question.trim()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_creation() {
        let bot = ManelismoBot::new();
        assert!(!bot.model.is_empty());
        assert!(!bot.base_url.is_empty());
    }

    #[test]
    fn test_fallback_response_mentions_initialization() {
        let response = fallback_response("Teste de pergunta");
        assert!(response.contains("inicializando"));
    }
}
