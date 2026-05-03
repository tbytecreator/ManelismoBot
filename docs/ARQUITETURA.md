# Arquitetura do Sistema - Bot do Manelismo

## Visão Geral

O Bot do Manelismo é um sistema de chatbot que simula a personalidade e o pensamento de Manoel Alves Ferreira Neto (TByteCreator). O sistema segue uma arquitetura de microserviços containerizada com separação clara entre backend e frontend.

## Diagrama de Arquitetura

```
┌─────────────────────────────────────────────────────────────┐
│                    Docker Container                          │
│                                                               │
│  ┌─────────────────┐         ┌──────────────────┐           │
│  │   Frontend      │         │   Backend        │           │
│  │   (React)       │◄────────┤   (Rust/Actix)   │           │
│  │   Port 3000     │   HTTP  │   Port 8080      │           │
│  └─────────────────┘         └──────────────────┘           │
│         │                            │                       │
│         │ Serve Static Files         │ Process Questions     │
│         │ User Interface             │ Generate Responses    │
│         │                            │                       │
└─────────────────────────────────────────────────────────────┘
         │                            │
         └────────────────┬───────────┘
                          │
                   User Interaction
```

## Componentes Principais

### 1. Backend (Rust + Actix-web)

**Localização:** `backend/`

**Responsabilidades:**
- Processar requisições HTTP POST em `/api/ask`
- Executar lógica de geração de respostas através do módulo `ManelismoBot`
- Servir health checks em `/api/health`
- Gerenciar CORS para comunicação com o frontend

**Estrutura de Módulos:**
```
src/
├── main.rs                 # Configuração principal do servidor
├── models.rs               # Modelos de dados (Request/Response)
├── manelismo_bot.rs        # Lógica central do bot
└── handlers/
    └── ask.rs              # Handler para requisições de perguntas
```

**Tecnologias:**
- Actix-web 4.x - Framework web assíncrono
- Rust 2021 edition
- Tokio para runtime assíncrono

### 2. Frontend (React + Node.js)

**Localização:** `frontend/`

**Responsabilidades:**
- Interface responsiva para interação com o usuário
- Renderização de mensagens de chat
- Requisições ao backend API
- Exibição de estado de carregamento e respostas

**Estrutura de Componentes:**
```
src/
├── App.js                  # Componente principal
├── App.css                 # Estilos principais
├── index.js                # Ponto de entrada
├── index.css               # Estilos globais
└── components/
    ├── ChatMessage.js      # Componente de mensagem
    ├── ChatMessage.css     # Estilos de mensagem
    ├── ChatInput.js        # Componente de entrada
    └── ChatInput.css       # Estilos de entrada
```

**Tecnologias:**
- React 18.x
- Node.js 18.x
- CSS3 com gradientes e animações

### 3. Containerização (Docker)

**Arquivos Docker:**
- `Dockerfile.backend` - Imagem do backend (Rust)
- `Dockerfile.frontend` - Imagem do frontend (Node.js)
- `docker-compose.yml` - Orquestração de containers

**Rede:**
- Bridge network chamada `manelismo-network`
- Permite comunicação entre containers

## Fluxo de Dados

```
1. Usuário digita pergunta no frontend (React)
   ↓
2. Frontend envia POST request para /api/ask (Backend)
   {
     "question": "Qual é o sentido da vida?"
   }
   ↓
3. Backend recebe requisição no handler ask.rs
   ↓
4. ManelismoBot gera resposta baseada na pergunta
   ↓
5. Backend retorna response JSON ao frontend
   {
     "answer": "Essa é uma das questões mais importantes...",
     "confidence": 0.85,
     "source": "ManelismoBot v0.1"
   }
   ↓
6. Frontend renderiza resposta como nova mensagem
   ↓
7. Usuário vê resposta na interface
```

## Endpoints da API

### POST /api/ask
Processa uma pergunta e retorna resposta do bot.

**Request:**
```json
{
  "question": "string"
}
```

**Response:**
```json
{
  "answer": "string",
  "confidence": 0.85,
  "source": "ManelismoBot v0.1"
}
```

### GET /api/health
Verificar saúde do serviço.

**Response:**
```json
{
  "status": "ok",
  "service": "Manelismo Bot Backend"
}
```

## Modelo de Dados - ManelismoBot

O módulo `manelismo_bot.rs` contém a lógica central:

**Características:**
- 5 traços de personalidade definidos
- Base de conhecimento com 5 fatos principais
- Geração de respostas contextual baseada em palavras-chave
- Resposta padrão para perguntas não categorizadas

**Lógica de Resposta:**
- Identifica palavras-chave na pergunta (who, sense, technology, future, youtube)
- Retorna respostas específicas para cada contexto
- Mantém personalidade consistente com Manoel Alves Ferreira Neto

## Escalabilidade e Futuros Melhoramentos

### Curto Prazo:
1. Integração com LLMs (OpenAI, Hugging Face)
2. Base de dados para armazenar histórico de conversas
3. Cache de respostas frequentes
4. Autenticação e rate limiting

### Médio Prazo:
1. Multi-language support
2. Análise de sentimentos
3. Persistência de sessões
4. Analytics e logging avançado

### Longo Prazo:
1. Deployment em cloud (AWS, Azure, GCP)
2. Auto-scaling baseado em carga
3. CI/CD pipeline
4. Monitoring e alertas

## Segurança

**Implementações Atuais:**
- CORS habilitado
- Input validation
- Error handling robusto

**Melhorias Recomendadas:**
- HTTPS/TLS
- API key authentication
- Rate limiting
- SQL injection prevention (se usar DB)
- DDOS protection

## Performance

**Características:**
- Backend assíncrono com Tokio
- Frontend otimizado com React
- Lazy loading de componentes
- CSS animations hardware-aceleradas

## Desenvolvimento Local

Ver [README.md](../README.md) para instruções de setup e execução.
