#!/bin/sh
set -e

OLLAMA_HOST_VALUE="${OLLAMA_HOST:-127.0.0.1:11434}"
MODEL_NAME="${OLLAMA_MODEL:-deepseek-r1:8b}"
OLLAMA_LOG_FILE="/tmp/ollama.log"
OLLAMA_PULL_LOG_FILE="/tmp/ollama-pull.log"

export OLLAMA_HOST="$OLLAMA_HOST_VALUE"
export OLLAMA_BASE_URL="${OLLAMA_BASE_URL:-http://$OLLAMA_HOST}"

echo "[startup] Iniciando Ollama em $OLLAMA_HOST..."
if ! command -v ollama >/dev/null 2>&1; then
  echo "[startup] ERRO: binario 'ollama' nao encontrado na imagem"
  exit 1
fi

ollama serve >"$OLLAMA_LOG_FILE" 2>&1 &
OLLAMA_PID=$!

READY=0
for i in $(seq 1 60); do
  if ! kill -0 "$OLLAMA_PID" >/dev/null 2>&1; then
    echo "[startup] ERRO: processo do Ollama encerrou antes de ficar pronto"
    if [ -f "$OLLAMA_LOG_FILE" ]; then
      echo "[startup] Ultimas linhas do log do Ollama:"
      tail -n 80 "$OLLAMA_LOG_FILE" || true
    fi
    break
  fi

  if curl -fsS "http://$OLLAMA_HOST/api/tags" >/dev/null 2>&1; then
    READY=1
    break
  fi
  sleep 1
done

if [ "$READY" -ne 1 ]; then
  echo "[startup] Ollama não ficou pronto a tempo. Backend iniciará com fallback."
  if [ -f "$OLLAMA_LOG_FILE" ]; then
    echo "[startup] Ultimas linhas do log do Ollama:"
    tail -n 80 "$OLLAMA_LOG_FILE" || true
  fi
else
  if ollama list | awk 'NR>1 {print $1}' | grep -q "^${MODEL_NAME}\(:.*\)\?$"; then
    echo "[startup] Modelo ja disponivel no Ollama: $MODEL_NAME"
  elif pgrep -af "ollama pull ${MODEL_NAME}" >/dev/null 2>&1; then
    echo "[startup] Download do modelo ja esta em andamento: $MODEL_NAME"
    echo "[startup] Acompanhe em: $OLLAMA_PULL_LOG_FILE"
  else
    echo "[startup] Iniciando download do modelo gratuito em background: $MODEL_NAME"
    ollama pull "$MODEL_NAME" >"$OLLAMA_PULL_LOG_FILE" 2>&1 &
    PULL_PID=$!
    echo "[startup] PID do download: $PULL_PID"
    echo "[startup] Log do download: $OLLAMA_PULL_LOG_FILE"
  fi
fi

echo "[startup] Iniciando backend Rust..."
exec ./target/release/manelismo_bot_backend
