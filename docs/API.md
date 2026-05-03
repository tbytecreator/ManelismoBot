# API Documentation - Bot do Manelismo

## Base URL

```
http://localhost:8080
```

## Endpoints

### 1. Ask Question

Processa uma pergunta e retorna a resposta do bot.

**Endpoint:** `POST /api/ask`

**Request Headers:**
```
Content-Type: application/json
```

**Request Body:**
```json
{
  "question": "Qual é o sentido da vida?"
}
```

**Response (200 OK):**
```json
{
  "answer": "Essa é uma das questões mais importantes da humanidade...",
  "confidence": 0.85,
  "source": "ManelismoBot v0.1"
}
```

**Response (400 Bad Request):**
```json
{
  "error": "Invalid request",
  "details": "Question cannot be empty"
}
```

**Exemplos de Palavras-Chave:**
- "quem você é?" → Resposta sobre identidade
- "qual sentido" → Resposta sobre propósito
- "tecnologia" → Resposta sobre tech e sociedade
- "futuro" → Resposta sobre perspectivas futuras
- "youtube/criação" → Resposta sobre criação de conteúdo

### 2. Health Check

Verifica se o backend está operacional.

**Endpoint:** `GET /api/health`

**Response (200 OK):**
```json
{
  "status": "ok",
  "service": "Manelismo Bot Backend"
}
```

## CORS

O backend aceita requisições de qualquer origem (CORS permissivo).

**Headers CORS Habilitados:**
- Access-Control-Allow-Origin: *
- Access-Control-Allow-Methods: GET, POST, OPTIONS
- Access-Control-Allow-Headers: Content-Type

## Rate Limiting

Atualmente não há rate limiting implementado. 

**Futuro:** Implementar rate limiting por IP.

## Error Handling

Todos os erros retornam status HTTP apropriado com mensagem de erro.

### Possíveis Códigos de Status

- `200 OK` - Requisição bem-sucedida
- `400 Bad Request` - Dados inválidos
- `500 Internal Server Error` - Erro no servidor

## Logging

Todas as requisições são registradas em log com timestamp:

```
[INFO] Received question: Qual é o sentido da vida?
[INFO] Generated response for question
```

## Autenticação

Não requer autenticação na versão atual.

## Versionamento

API versão: `0.1.0`

Endpoint base: `/api` (pode ser expandido para `/api/v1` em versões futuras)

## Exemplos com cURL

### Ask Question

```bash
curl -X POST http://localhost:8080/api/ask \
  -H "Content-Type: application/json" \
  -d '{"question":"Quem você é?"}'
```

### Health Check

```bash
curl http://localhost:8080/api/health
```

## Exemplos com JavaScript/Fetch

### Ask Question

```javascript
const response = await fetch('http://localhost:8080/api/ask', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    question: 'Qual é o sentido da vida?'
  })
});

const data = await response.json();
console.log(data.answer);
```

### Health Check

```javascript
const response = await fetch('http://localhost:8080/api/health');
const data = await response.json();
console.log(data.status); // "ok"
```

## Roadmap

- [ ] Integração com LLM para respostas mais sofisticadas
- [ ] Base de dados para histórico de conversas
- [ ] Autenticação com API keys
- [ ] Rate limiting por usuário
- [ ] Webhooks para integração com outros sistemas
- [ ] WebSocket para real-time messaging
