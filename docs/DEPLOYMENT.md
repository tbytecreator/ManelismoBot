# Guia de Deployment

## Deployment com Docker

### Pré-requisitos

- Docker instalado
- Docker Compose instalado
- Docker Desktop rodando

### Build e Execução Local

#### 1. Build com Docker Compose (Recomendado)

```bash
# Na raiz do projeto
docker compose build
docker compose up
```

Observação: na primeira execução, o backend baixa automaticamente o modelo gratuito `deepseek-r1:8b`.

Backend estará disponível em `http://localhost:8080`
Frontend estará disponível em `http://localhost:3000`

#### 2. Build Individual

**Backend:**
```bash
docker build -f Dockerfile.backend -t manelismo-bot-backend:latest .
docker run -p 8080:8080 manelismo-bot-backend:latest
```

**Frontend:**
```bash
docker build -f Dockerfile.frontend -t manelismo-bot-frontend:latest .
docker run -p 3000:3000 manelismo-bot-frontend:latest
```

#### 3. Build Multi-stage (Legado)

O `Dockerfile` raiz é legado e não sobe o LLM local automaticamente.
Para esta versão com IA local, use `Dockerfile.backend` + `Dockerfile.frontend` (ou `docker compose`).

### Parar Containers

```bash
# Se usando docker compose
docker compose down

# Se usando docker run
docker stop container-id
docker rm container-id
```

### Verificar Status

```bash
docker compose ps
docker logs container-name
docker stats
```

## Deployment em Cloud

### AWS ECS

1. **Criar ECR Repository:**
```bash
aws ecr create-repository --repository-name manelismo-bot
```

2. **Tag e push da imagem:**
```bash
docker tag manelismo-bot:latest [AWS_ACCOUNT].dkr.ecr.[REGION].amazonaws.com/manelismo-bot:latest
docker push [AWS_ACCOUNT].dkr.ecr.[REGION].amazonaws.com/manelismo-bot:latest
```

3. **Criar Task Definition e Service no ECS**

### Google Cloud Run

```bash
# Build e push
gcloud builds submit --tag gcr.io/PROJECT-ID/manelismo-bot

# Deploy
gcloud run deploy manelismo-bot \
  --image gcr.io/PROJECT-ID/manelismo-bot \
  --platform managed \
  --region us-central1
```

### Azure Container Instances

```bash
# Push para Azure Container Registry
az acr build --registry myRegistry --image manelismo-bot:latest .

# Deploy
az container create \
  --resource-group myResourceGroup \
  --name manelismo-bot-container \
  --image myRegistry.azurecr.io/manelismo-bot:latest
```

## Environment Variables

### Backend (.env)

```
RUST_LOG=info
PORT=8080
ALLOWED_ORIGINS=http://localhost:3000
OLLAMA_BASE_URL=http://127.0.0.1:11434
OLLAMA_HOST=127.0.0.1:11434
OLLAMA_MODEL=tinyllama:1.1b
```

### Frontend (.env)

```
REACT_APP_API_URL=http://backend:8080
```

## Health Checks

### Docker Compose Health Check

O `docker-compose.yml` inclui health checks:

```bash
docker compose ps
# Verificar status (healthy/starting/unhealthy)
```

### Manual Health Check

```bash
curl http://localhost:8080/api/health
```

## Monitoring e Logs

### Docker Logs

```bash
# Logs do backend
docker logs manelismo-bot-backend

# Logs do frontend
docker logs manelismo-bot-frontend

# Seguir logs em tempo real
docker logs -f container-name
```

### Estrutura de Logs

Backend: `RUST_LOG=info` (arquivo: geralmente stdout)
Frontend: Console browser (DevTools -> Console)

## Performance e Scaling

### Local Optimization

```dockerfile
# Backend: Multi-stage build reduz tamanho
FROM rust:1.75 as builder
# ... build ...
FROM debian:bookworm-slim
# ... apenas runtime ...
```

### Scaling Horizontal

Com kubernetes:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: manelismo-bot-backend
spec:
  replicas: 3
  selector:
    matchLabels:
      app: manelismo-bot-backend
  template:
    metadata:
      labels:
        app: manelismo-bot-backend
    spec:
      containers:
      - name: backend
        image: manelismo-bot-backend:latest
        ports:
        - containerPort: 8080
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
```

## CI/CD Pipeline

### GitHub Actions Example

```yaml
name: Build and Deploy

on:
  push:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Build Docker image
        run: docker build -t manelismo-bot:latest .
      - name: Push to registry
        run: docker push manelismo-bot:latest
```

## Troubleshooting

### Container não inicia

```bash
docker logs container-name
# Verificar mensagens de erro
```

### Port already in use

```bash
# Liberar porta
lsof -i :8080
kill -9 PID

# Ou usar porta diferente
docker run -p 9090:8080 manelismo-bot
```

### Out of memory

```bash
# Aumentar memória disponível para Docker
docker run -m 1g manelismo-bot
```

### Network issues

```bash
# Verificar network
docker network ls
docker network inspect manelismo-network
```

## Backup e Recovery

### Backup de dados (se aplicável)

```bash
docker exec container-name tar czf - /data > backup.tar.gz
```

### Recovery

```bash
docker exec container-name tar xzf /tmp/backup.tar.gz
```

## Security

### Best Practices

1. Use image scanning: `docker scan image-name`
2. Update base images regularmente
3. Use secrets manager para credentials
4. Implement rate limiting
5. Use HTTPS/TLS em produção

### Secrets Management

```bash
# Docker Secrets (Swarm)
echo "api_key" | docker secret create api_key -

# Environment variables (com precaução)
docker run -e API_KEY=value manelismo-bot
```

## Documentação de Rollback

### Versioning

```bash
docker tag manelismo-bot:latest manelismo-bot:v1.0.0
docker push manelismo-bot:v1.0.0
```

### Rollback

```bash
docker compose down
docker pull manelismo-bot:v0.9.9
docker compose up
```
