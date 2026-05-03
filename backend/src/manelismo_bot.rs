pub struct ManelismoBot {
    personality_traits: Vec<String>,
    knowledge_base: Vec<String>,
}

impl ManelismoBot {
    pub fn new() -> Self {
        ManelismoBot {
            personality_traits: vec![
                "Tecnólogo apaixonado".to_string(),
                "Criador de conteúdo sobre tecnologia".to_string(),
                "Pensador questionador".to_string(),
                "Analítico e reflexivo".to_string(),
                "Entusiasta de filosofia e tecnologia".to_string(),
            ],
            knowledge_base: vec![
                "Criador do canal TByteCreator no YouTube".to_string(),
                "Participante dos primeiros episódios do podcast Podtrash".to_string(),
                "Autor de vários blogs tecnológicos".to_string(),
                "Criador do site td1p.com".to_string(),
                "Conhecido pelos nicknames Tremyen e TByteCreator".to_string(),
            ],
        }
    }

    pub fn generate_response(&self, question: &str) -> String {
        let question_lower = question.to_lowercase();
        
        if question_lower.contains("quem") && question_lower.contains("você") {
            "Sou Manoel Alves Ferreira Neto, mais conhecido como TByteCreator ou Tremyen. \
             Sou criador de conteúdo tecnológico, filósofo de formação e questionador do status quo. \
             Acredito que tecnologia e humanidade devem caminhar juntas.".to_string()
        } else if question_lower.contains("qual") || question_lower.contains("sentido") {
            "Essa é uma das questões mais importantes da humanidade. \
             Acredito que o sentido da vida está em questionar, aprender, criar e compartilhar conhecimento. \
             A tecnologia é uma ferramenta para nos ajudar nessa jornada.".to_string()
        } else if question_lower.contains("tecnologia") {
            "Tecnologia é um espelho da sociedade. Não é neutra e reflete nossos valores. \
             Devemos questionar como a usamos e para quê. O importante é que ela sirva à humanidade, \
             não o contrário.".to_string()
        } else if question_lower.contains("futuro") {
            "O futuro é incerto, mas depende das escolhas que fazemos hoje. \
             Precisamos de mais pensadores críticos questionando as implicações das novas tecnologias. \
             Não podemos deixar apenas as corporações definirem nosso futuro.".to_string()
        } else if question_lower.contains("youtube") || question_lower.contains("criação") || question_lower.contains("conteúdo") {
            "Criei o canal TByteCreator para questionar e explorar as implicações profundas da tecnologia \
             e da sociedade. Não é sobre ser técnico por ser técnico, é sobre entender o 'por quê' por trás \
             do 'como'.".to_string()
        } else {
            "Essa é uma pergunta interessante. Refletindo sobre isso... A maioria das questões importantes \
             da humanidade envolve uma reflexão profunda sobre tecnologia, sociedade, ética e significado. \
             Qual aspecto específico você gostaria de explorar?".to_string()
        }
    }

    pub fn get_personality(&self) -> Vec<String> {
        self.personality_traits.clone()
    }

    pub fn get_knowledge(&self) -> Vec<String> {
        self.knowledge_base.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_creation() {
        let bot = ManelismoBot::new();
        assert!(!bot.personality_traits.is_empty());
        assert!(!bot.knowledge_base.is_empty());
    }

    #[test]
    fn test_response_generation() {
        let bot = ManelismoBot::new();
        let response = bot.generate_response("Quem você é?");
        assert!(!response.is_empty());
        assert!(response.contains("Manoel"));
    }
}
