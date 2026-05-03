#!/bin/sh
set -e

OLLAMA_HOST_VALUE="${OLLAMA_HOST:-127.0.0.1:11434}"
MODEL_NAME="${OLLAMA_MODEL:-deepseek-r1:8b}"

export OLLAMA_HOST="$OLLAMA_HOST_VALUE"

echo "[startup] Iniciando Ollama em $OLLAMA_HOST..."
ollama serve >/tmp/ollama.log 2>&1 &

READY=0
for i in $(seq 1 60); do
  if curl -fsS "http://$OLLAMA_HOST/api/tags" >/dev/null 2>&1; then
    READY=1
    break
  fi
  sleep 1
done

if [ "$READY" -ne 1 ]; then
  echo "[startup] Ollama não ficou pronto a tempo. Backend iniciará com fallback."
else
  if ! ollama list | awk 'NR>1 {print $1}' | grep -q "^${MODEL_NAME}\(:.*\)\?$"; then
    echo "[startup] Iniciando download do modelo gratuito em background: $MODEL_NAME"
    ollama pull "$MODEL_NAME" >/tmp/ollama-pull.log 2>&1 &
  fi
fi

echo "[startup] Iniciando backend Rust..."
exec ./target/release/manelismo_bot_backend
