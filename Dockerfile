# Build stage para backend
FROM rust:1.88 as backend_builder

WORKDIR /app/backend

# Copiar arquivos de configuração
COPY backend/Cargo.toml ./Cargo.toml
COPY backend/src ./src

# Build do backend
RUN cargo build --release

# Stage 2: Build do frontend
FROM node:18-alpine as frontend_builder

WORKDIR /app/frontend

# Copiar package files
COPY frontend/package.json ./
COPY frontend/package-lock.json* ./

# Instalar dependências
RUN npm ci

# Copiar código fonte
COPY frontend/src ./src
COPY frontend/public ./public

# Build do frontend
RUN npm run build

# Stage 3: Runtime
FROM node:18-alpine

WORKDIR /app

# Instalar runtime para Rust
RUN apk add --no-cache ca-certificates

# Copiar backend compilado
COPY --from=backend_builder /app/backend/target/release/manelismo_bot_backend /app/backend/

# Copiar frontend compilado
COPY --from=frontend_builder /app/frontend/build /app/public

# Instalar express para servir o frontend
RUN npm init -y && npm install express

# Criar script para iniciar ambos os serviços
RUN echo '#!/bin/sh\n/app/backend/manelismo_bot_backend &\nnode -e "const express = require('"'"'express'"'"'); const app = express(); app.use(express.static('"'"'/app/public'"'"')); app.get('"'"'*'"'"', (req, res) => res.sendFile('"'"'/app/public/index.html'"'"')); app.listen(3000, () => console.log('"'"'Frontend listening on port 3000'"'"'));"' > /app/start.sh && chmod +x /app/start.sh

EXPOSE 8080 3000

CMD ["/app/start.sh"]
