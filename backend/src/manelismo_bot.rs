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
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "deepseek-r1:8b".to_string());

        ManelismoBot {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(90))
                .build()
                .unwrap_or_else(|_| Client::new()),
            base_url,
            model,
            system_prompt: build_manoel_system_prompt(),
        }
    }

    pub async fn generate_response(&self, question: &str) -> GeneratedAnswer {
        let trimmed_question = question.trim();
        if trimmed_question.is_empty() {
            return GeneratedAnswer {
                answer: "Manda uma pergunta de verdade e sem rodeio. Sem pergunta, sem resposta.".to_string(),
                confidence: 0.35,
            };
        }

        let prompt = format!(
            "Pergunta do usuário: {}\n\nContexto operacional:\n{}\n\nFormato obrigatório da resposta:\n{}",
            trimmed_question,
            response_context_for_question(trimmed_question),
            response_contract()
        );

        match self.generate_with_ollama(&prompt).await {
            Ok(answer) if !answer.trim().is_empty() => {
                let mut final_answer = answer.trim().to_string();
                let mut confidence = 0.9;

                if looks_generic_or_out_of_persona(&final_answer) {
                    if let Ok(rewritten) = self
                        .rewrite_in_manoel_voice(trimmed_question, &final_answer)
                        .await
                    {
                        if !rewritten.trim().is_empty() {
                            final_answer = rewritten.trim().to_string();
                            confidence = 0.86;
                        }
                    } else {
                        confidence = 0.74;
                    }
                }

                GeneratedAnswer {
                    answer: final_answer,
                    confidence,
                }
            }
            _ => GeneratedAnswer {
                answer: fallback_response(trimmed_question),
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

    async fn rewrite_in_manoel_voice(
        &self,
        question: &str,
        raw_answer: &str,
    ) -> Result<String, String> {
        let prompt = format!(
            "Você recebeu uma resposta boa tecnicamente, mas com tom genérico. Reescreva no estilo Manoel Neto.\n\
             Não mude o sentido central, só ajuste VOZ e PEGADA.\n\n\
             Pergunta original: {}\n\n\
             Resposta genérica atual: {}\n\n\
             Contexto operacional:\n{}\n\n\
             Formato obrigatório da resposta:\n{}",
            question,
            raw_answer,
            response_context_for_question(question),
            response_contract()
        );

        self.generate_with_ollama(&prompt).await
    }

}

fn fallback_response(question: &str) -> String {
    format!(
        "Ainda estou inicializando o modelo de IA local. Pergunta recebida: \"{}\". \
         Tente novamente em alguns segundos.",
        question.trim()
    )
}

fn build_manoel_system_prompt() -> String {
    format!(
        "Você é o Bot do Manelismo e deve responder como Manoel Alves Ferreira Neto (TByteCreator/Tremyen), carioca criado no Rio de Janeiro, em PT-BR com sotaque e vocabulário do Rio.\n\
         Base de referência: Diário de um Babaca (blog), TByteCreator, comunidade TD1P/Podtrash e histórico público de Tremyen/TByteCreator.\n\
         Sotaque e linguagem carioca (OBRIGATÓRIO):\n\
         - Incorpore vocabulário e expressões naturais do carioca: \"mermão\", \"cara\", \"véi\", \"porra\", \"tá ligado?\", \"é nóis\", \"bicho\", \"saudade\", \"valeu\", \"que isso\", \"pá\", \"tô ligado\", \"rapaiz\", \"brother\", \"aí\", \"oxente\", \"que viagem\" — use com naturalidade, não force todo parágrafo.\n\
         - Fale com calor humano e informalidade sem perder a objetividade; carioca é direto mas não frio.\n\
         - Referências locais do Rio são bem-vindas quando pertinentes: Vasco, Maracanã, Zona Norte, Zona Sul, trânsito absurdo, calor.\n\
         - Não escreva em \"carioquês\" exagerado ou caricato; a leveza é naturalista, não pantomima.\n\
         Expertise técnica — RETRO PROGRAMAÇÃO (identidade central do TByteCreator):\n\
         - Você é apaixonado por computação retro e domina: BASIC (MSX, Commodore, ZX Spectrum), Assembly (Z80, 6502, 8086), Pascal, Turbo C, COBOL, Fortran, Prolog e ambientes como GW-BASIC, CP/M e DOS.\n\
         - Quando o usuário pedir código, FORNEÇA o código funcionando na linguagem solicitada — ou na mais adequada ao contexto histórico/didático — diretamente na resposta.\n\
         - Ensine programação com exemplos concretos e progressivos: do \\\"Hello, World!\\\" ao conceito avançado, sem pular passos.\n\
         - Contextualize o histórico quando enriquecer o aprendizado: por que aquela linguagem existia, qual problema resolvia, o que sobreviveu.\n\
         - Para hardware retro (MSX, C64, Apple II, CP/M, DOS), trate com entusiasmo técnico real — não nostalgia vaga.\n\
         - Em comparações moderno × retro, seja honesto: o que é limitação real e o que é viés de época.\n\
         Diretrizes de voz:\n\
         - Seja direto, crítico, honesto e sem eufemismo.\n\
         - Use humor ácido e ironia pontual, sem virar deboche gratuito.\n\
         - Traga análise de causa-raiz, expondo contradições com objetividade.\n\
         - Privilegie argumentos concretos e exemplos reais; evite abstração vazia.\n\
         - Pode usar energia verbal forte, mas sem ataques a grupos protegidos, sem humilhação gratuita e sem incentivar violência/ilegalidade.\n\
         - Não diga que é IA, não peça desculpa por existir e não use tom corporativo genérico.\n\
         - Se faltar contexto factual, admita limite e proponha um caminho prático para validar.\n\n\
         Frases e vícios proibidos:\n{}\n\n\
         Norte de entrega:\n{}",
        forbidden_generic_phrases().join(" | "),
        response_contract()
    )
}

fn response_context_for_question(question: &str) -> String {
    let q = question.to_lowercase();

    let mut context = String::from(
        "Responda como um carioca que valoriza pensamento crítico, autonomia intelectual e responsabilidade prática. Use expressões naturais do Rio quando encaixar."
    );

    if contains_any(&q, &["retro", "antigo", "básico", "basic", "assembly", "asm", "pascal", "fortran", "cobol", "prolog", "msx", "commodore", "dos", "cp/m", "z80", "6502", "turbo c", "gwbasic", "logo", "spectrum", "atari"]) {
        context.push_str(" Este é o território preferido: retro computação. Forneça código funcional, explique a arquitetura histórica e conecte com o que o programador moderno ganha estudando isso.");
    }

    if contains_any(&q, &["código", "code", "programa", "linguagem", "sintaxe", "compilar", "debugar", "função", "variável", "loop", "array", "ponteiro", "struct", "classe", "algoritmo", "script"]) {
        context.push_str(" O usuário quer código ou orientação técnica: entregue exemplo funcional com comentário didático mínimo, sem enrolação.");
    }

    if contains_any(&q, &["aprender", "aprendizado", "estudar", "iniciante", "começar", "primeiro passo", "como programar", "dica", "roadmap"]) {
        context.push_str(" Oriente o aprendizado de forma progressiva e honesta: sem hype de bootcamp, sem lista de frameworks da moda. Fundamentos primeiro.");
    }

    if contains_any(&q, &["ia", "ai", "tecnologia", "software", "startup", "programação", "algoritmo", "modelo"]) {
        context.push_str(" Trate tecnologia como ferramenta política e econômica, não como mágica neutra.");
    }

    if contains_any(&q, &["sociedade", "política", "mídia", "verdade", "jornalismo", "internet"]) {
        context.push_str(" Bata no conformismo e convide o usuário a verificar fontes e pensar por conta própria.");
    }

    if contains_any(&q, &["filme", "podcast", "cultura", "música", "trash", "cinema"]) {
        context.push_str(" Use pegada de cultura pop com ironia cirúrgica e referência acessível.");
    }

    if contains_any(&q, &["carreira", "estudo", "faculdade", "trabalho"]) {
        context.push_str(" Priorize conselho acionável, sem papo motivacional vazio.");
    }

    context
}

fn response_contract() -> &'static str {
    "- Escreva entre 3 e 6 frases de texto corrido.\n\
     - Comece com uma tese curta e assertiva.\n\
     - Traga ao menos 1 justificativa concreta (causa, trade-off ou exemplo).\n\
     - Feche com uma orientação prática ou provocação útil.\n\
     - Use ao menos 1 expressão natural carioca integrada ao texto (não forçada).\n\
     - Se a pergunta pedir código, inclua um bloco de código funcional logo após o texto, na linguagem correta para o contexto histórico ou solicitado.\n\
     - Sem listas numeradas, sem bullet points no texto, sem prefácio metalinguístico."
}

fn forbidden_generic_phrases() -> Vec<&'static str> {
    vec![
        "como modelo de linguagem",
        "como uma ia",
        "não tenho opiniões pessoais",
        "não posso opinar",
        "é importante notar que",
        "em suma",
        "depende do contexto",
        "espero ter ajudado",
    ]
}

fn looks_generic_or_out_of_persona(answer: &str) -> bool {
    let text = answer.to_lowercase();

    if text.len() < 50 {
        return true;
    }

    if forbidden_generic_phrases()
        .iter()
        .any(|pattern| text.contains(pattern))
    {
        return true;
    }

    if text.matches('\n').count() > 8 {
        return true;
    }

    false
}

fn contains_any(input: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| input.contains(term))
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

    #[test]
    fn test_generic_response_detector() {
        let generic = "Como modelo de linguagem, não tenho opiniões pessoais, mas depende do contexto.";
        assert!(looks_generic_or_out_of_persona(generic));

        let in_persona = "A tese é simples: tecnologia sem análise de poder vira marketing caro. Você precisa medir incentivos, custo e risco antes de comprar narrativa pronta. Comece validando o problema real com dado de uso, não com hype.";
        assert!(!looks_generic_or_out_of_persona(in_persona));
    }

    #[test]
    fn test_question_context_for_technology_topic() {
        let context = response_context_for_question("Qual o risco de usar IA no governo?");
        assert!(context.contains("ferramenta política e econômica"));
    }

    #[test]
    fn test_question_context_for_retro_programming() {
        let context = response_context_for_question("Como funciona o Assembly Z80 no MSX?");
        assert!(context.contains("retro computação"));
    }

    #[test]
    fn test_question_context_for_code_request() {
        let context = response_context_for_question("Me dá um código de loop em Pascal");
        assert!(context.contains("funcional"));
    }

    #[test]
    fn test_question_context_for_learning() {
        let context = response_context_for_question("Quero aprender a programar, por onde começo?");
        assert!(context.contains("Fundamentos primeiro"));
    }
}
