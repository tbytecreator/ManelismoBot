# Guia de Desenvolvimento

## Setup Local

> Caminho da raiz do projeto (relativo): `./`

### Pré-requisitos

- Rust 1.75+ ([instalar](https://rustup.rs/))
- Node.js 18+ ([instalar](https://nodejs.org/))
- npm 9+
- Docker Desktop (opcional, para containerização)

### Backend Setup

1. **Navegar para pasta do backend:**
```bash
cd backend
```

2. **Instalar dependências (automático com cargo):**
```bash
cargo build
```

3. **Executar backend:**
```bash
cargo run
```

Backend estará disponível em `http://localhost:8080`

### Frontend Setup

1. **Navegar para pasta do frontend:**
```bash
cd frontend
```

2. **Instalar dependências:**
```bash
npm install
```

3. **Executar frontend em desenvolvimento:**
```bash
npm start
```

Frontend estará disponível em `http://localhost:3000`

## Estrutura de Projeto

```
ManelismoBot/ManelismoBot/
├── backend/                 # Backend em Rust
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── models.rs
│   │   ├── manelismo_bot.rs
│   │   └── handlers/
│   │       └── ask.rs
│   └── target/             # Build output
│
├── frontend/                # Frontend em React
│   ├── package.json
│   ├── public/
│   │   └── index.html
│   ├── src/
│   │   ├── App.js
│   │   ├── App.css
│   │   ├── index.js
│   │   ├── index.css
│   │   └── components/
│   │       ├── ChatMessage.js
│   │       ├── ChatMessage.css
│   │       ├── ChatInput.js
│   │       └── ChatInput.css
│   └── build/              # Build output
│
├── docs/                    # Documentação
│   ├── ARQUITETURA.md
│   ├── API.md
│   ├── DESENVOLVIMENTO.md
│   └── DEPLOYMENT.md
│
├── Dockerfile              # Multi-stage build
├── Dockerfile.backend
├── Dockerfile.frontend
├── docker-compose.yml
└── requisitos.md           # Requisitos originais
```

## Workflow de Desenvolvimento

### 1. Modificar Backend

```bash
cd backend
# Fazer alterações em src/
cargo check              # Verificar sintaxe
cargo test              # Rodar testes
cargo run               # Executar localmente
```

### 2. Modificar Frontend

```bash
cd frontend
# Fazer alterações em src/
npm run build           # Build para produção
npm start               # Desenvolvimento com hot-reload
```

### 3. Testar Comunicação

```bash
# Terminal 1: Backend
cd backend && cargo run

# Terminal 2: Frontend
cd frontend && npm start

# Terminal 3: Testar API
curl -X POST http://localhost:8080/api/ask \
  -H "Content-Type: application/json" \
  -d '{"question":"Quem você é?"}'
```

## Testes

### Backend

```bash
cd backend

# Rodar todos os testes
cargo test

# Rodar com output
cargo test -- --nocapture

# Rodar teste específico
cargo test test_bot_creation
```

### Frontend

```bash
cd frontend

# Rodar testes
npm test

# Build
npm run build
```

## Linting e Formatting

### Backend (Rust)

```bash
cd backend

# Format
cargo fmt

# Lint
cargo clippy
```

### Frontend (JavaScript)

```bash
cd frontend

# Format (usando prettier se instalado)
npx prettier --write src/

# Lint (usando eslint se instalado)
npx eslint src/
```

## Build para Produção

### Backend

```bash
cd backend
cargo build --release
# Output: target/release/manelismo_bot_backend
```

### Frontend

```bash
cd frontend
npm run build
# Output: build/ (pasta pronta para deploy)
```

## Debugging

### Backend

```bash
# Com logs detalhados
RUST_LOG=debug cargo run

# Diferentes níveis
RUST_LOG=error,manelismo_bot=debug cargo run
```

### Frontend

```bash
# Chrome DevTools: F12
# React DevTools: https://github.com/facebook/react-devtools
npm start
```

## Adicionar Novas Dependências

### Backend

```bash
cd backend
cargo add nome-da-dependency
```

### Frontend

```bash
cd frontend
npm install nome-do-package
```

## Variáveis de Ambiente

### Backend

Criar `.env` na pasta `backend/`:
```
RUST_LOG=info
PORT=8080
```

### Frontend

Criar `.env` na pasta `frontend/`:
```
REACT_APP_API_URL=http://localhost:8080
```

## Troubleshooting

### Backend não inicia
- Verificar porta 8080 está livre
- Instalar Rust latest: `rustup update`

### Frontend não conecta ao backend
- Verificar CORS no backend
- Verificar URL da API em `src/App.js`
- Verificar se backend está rodando

### Docker build falha
- Limpar cache: `docker system prune -a`
- Verificar internet connection
- Verificar Docker installation

## Performance Tips

### Backend
- Use `cargo build --release` para produção
- Mantenha async/await para I/O
- Implemente caching para respostas

### Frontend
- Use React.memo para componentes
- Lazy load componentes grandes
- Otimize imagens
- Minify CSS/JS

## Contributing Guidelines

1. Create feature branch: `git checkout -b feature/nome-feature`
2. Make changes with tests
3. Ensure formatting: `cargo fmt` e `npm run build`
4. Commit with clear message
5. Push e create pull request

## Recursos Úteis

- [Rust Book](https://doc.rust-lang.org/book/)
- [Actix-web Docs](https://actix.rs/)
- [React Docs](https://react.dev/)
- [Manoel Alves References](../requisitos.md)
