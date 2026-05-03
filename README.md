# Bot do Manelismo 🤖

Chatbot que simula a personalidade e pensamento de **Manoel Alves Ferreira Neto** (TByteCreator/Tremyen), criador de conteúdo tecnológico e pensador questionador.

## ✨ Características

- 🎯 Interface web responsiva e intuitiva
- ⚡ Backend assíncrono em Rust com Actix-web
- 🚀 Frontend moderno em React 18
- 🐳 Totalmente containerizado com Docker
- 📚 Documentação completa
- 🧠 Respostas por IA com LLM local (Ollama + deepseek-r1:8b)
- 💬 Chat interativo em tempo real

## 📋 Pré-requisitos

### Para execução com Docker (Recomendado)

- Docker 20.10+
- Docker Compose 2.0+
- Conexão com internet no primeiro start (download inicial do modelo)

### Para desenvolvimento local

- Rust 1.88+ ([instalar](https://rustup.rs/))
- Node.js 18+ e npm 9+ ([instalar](https://nodejs.org/))
- Git

## 🚀 Quick Start com Docker

### Opção 1: Docker Compose (Mais Fácil)

```bash
# Clone ou navegue para a pasta do projeto
cd ManelismoBot

# Build e inicie os containers
docker compose up --build

# Ou em background
docker compose up -d --build

# Ver logs do backend (inclui download inicial do modelo na primeira execução)
docker compose logs -f backend
```

Acesse:

- Frontend: <http://localhost:3000>
- Backend API: <http://localhost:8080>
- Health check: <http://localhost:8080/api/health>

Observação: na primeira execução, o backend baixa automaticamente o modelo gratuito `deepseek-r1:8b`, então o primeiro start pode levar alguns minutos.

### Opção 2: Docker Build Manual

**Build:**

```bash
docker build -f Dockerfile.backend -t manelismo-bot-backend:latest .
docker build -f Dockerfile.frontend -t manelismo-bot-frontend:latest .
```

**Execute Backend:**

```bash
docker run -p 8080:8080 manelismo-bot-backend:latest
```

**Execute Frontend (novo terminal):**

```bash
docker run -p 3000:3000 manelismo-bot-frontend:latest
```

### Parar Containers

```bash
# Com docker compose
docker compose down

# Com docker run
docker stop container-id
docker rm container-id
```

## 💻 Desenvolvimento Local

### Setup Backend

```bash
cd backend

# Build
cargo build

# Executar (modo desenvolvimento)
cargo run

# Com logs detalhados
RUST_LOG=debug cargo run

# Rodar testes
cargo test
```

Backend estará em: <http://localhost:8080>

### Setup Frontend

```bash
cd frontend

# Instalar dependências
npm install

# Modo desenvolvimento (com hot-reload)
npm start

# Build para produção
npm run build

# Rodar testes
npm test
```

Frontend estará em: <http://localhost:3000>

## 📚 Documentação

Navegue na pasta `docs/` para documentação detalhada:

- **[ARQUITETURA.md](docs/ARQUITETURA.md)** - Arquitetura completa do sistema
- **[API.md](docs/API.md)** - Documentação de endpoints e exemplos
- **[DESENVOLVIMENTO.md](docs/DESENVOLVIMENTO.md)** - Guia para desenvolvimento local
- **[DEPLOYMENT.md](docs/DEPLOYMENT.md)** - Guia de deployment e produção

## 📡 API Endpoints

### POST /api/ask

Faz uma pergunta ao bot.

**Request:**

```json
{
  "question": "Qual é o sentido da vida?"
}
```

**Response:**

```json
{
  "answer": "Essa é uma das questões mais importantes...",
  "confidence": 0.88,
  "source": "ManelismoBot v0.1"
}
```

### GET /api/health

Verifica saúde do serviço.

**Response:**

```json
{
  "status": "ok",
  "service": "Manelismo Bot Backend"
}
```

## 🎯 Exemplos de Uso

### Com cURL

```bash
# Fazer pergunta
curl -X POST http://localhost:8080/api/ask \
  -H "Content-Type: application/json" \
  -d '{"question":"Quem você é?"}'

# Health check
curl http://localhost:8080/api/health
```

### Com JavaScript

```javascript
const response = await fetch('http://localhost:8080/api/ask', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ question: 'Qual é o sentido da vida?' })
});

const data = await response.json();
console.log(data.answer);
```

## 🏗️ Estrutura do Projeto

```ascii
ManelismoBot/
├── backend/                    # Backend Rust + Actix-web
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs            # Servidor HTTP
│   │   ├── models.rs          # Modelos de dados
│   │   ├── manelismo_bot.rs   # Lógica do bot
│   │   └── handlers/          # Handlers de requisições
│   └── target/                # Build output
│
├── frontend/                   # Frontend React
│   ├── package.json
│   ├── public/
│   ├── src/
│   │   ├── App.js             # Componente principal
│   │   ├── App.css            # Estilos
│   │   └── components/        # Componentes React
│   └── build/                 # Build output
│
├── docs/                       # Documentação
│   ├── ARQUITETURA.md
│   ├── API.md
│   ├── DESENVOLVIMENTO.md
│   └── DEPLOYMENT.md
│
├── Dockerfile                  # Multi-stage build
├── Dockerfile.backend          # Backend only
├── Dockerfile.frontend         # Frontend only
├── docker-compose.yml          # Docker Compose config
├── .dockerignore               # Docker ignore rules
└── README.md                   # Este arquivo
```

## 🔧 Variáveis de Ambiente

### Backend

Criar arquivo `backend/.env`:

```ascii
RUST_LOG=info
PORT=8080
OLLAMA_BASE_URL=http://127.0.0.1:11434
OLLAMA_HOST=127.0.0.1:11434
OLLAMA_MODEL=deepseek-r1:8b
```

### Frontend

Criar arquivo `frontend/.env`:

```ascii
REACT_APP_API_URL=http://localhost:8080
```

## 🧪 Testes

### Backend

```bash
cd backend
cargo test              # Todos os testes
cargo test -- --show-output  # Com output
```

### Frontend

```bash
cd frontend
npm test
npm run build  # Build para produção
```

## 📖 Sobre Manoel Alves Ferreira Neto

Manoel Alves Ferreira Neto é:

- **Criador** do canal YouTube [TByteCreator](https://www.youtube.com/@tbytecreator)
- **Participante** dos primeiros episódios do podcast Podtrash
- **Autor** de diversos blogs tecnológicos
- **Criador** do site [td1p.com](https://td1p.com/)
- **Conhecido** pelos nicknames: **Tremyen** e **TByteCreator**

Seus trabalhos:

- [Diário do Babaca](https://diariodobabaca.blogspot.com/) - Blog tecnológico
- [Tremyen Facebook](https://www.facebook.com/tremyen/)
- [Podtrash Ep. 22](https://td1p.com/podtrash-22-fica-frio-e-de-baunilha/)
- [YouTube Playlist](https://www.youtube.com/watch?v=8yN3AnNqE4U)

## 🚀 Próximos Passos e Melhorias

### Curto Prazo

- [ ] Integração com modelos de IA (OpenAI, Hugging Face)
- [ ] Base de dados para histórico de conversas
- [ ] Cache de respostas frequentes
- [ ] Autenticação com API keys

### Médio Prazo

- [ ] Suporte multilíngue
- [ ] Análise de sentimentos
- [ ] Persistência de sessões de usuário
- [ ] Analytics e logging avançado

### Longo Prazo

- [ ] Deploy em cloud (AWS, Azure, GCP)
- [ ] Auto-scaling com Kubernetes
- [ ] CI/CD pipeline automático
- [ ] Monitoring e alertas em tempo real

## 🤝 Contribuindo

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📝 Licença

Este projeto está sob a MIT License - veja o arquivo LICENSE para detalhes.

## 📞 Contato

Para perguntas, sugestões ou feedback:

- Consulte a [Documentação](docs/)
- Abra uma issue no repositório
- Visite os sites de referência (links acima)

## 🙏 Agradecimentos

Inspirado pela obra, pensamento e criações de **Manoel Alves Ferreira Neto**.

---

**Versão:** 0.1.0  
**Última atualização:** Maio 2026  
**Status:** Em desenvolvimento ativo
