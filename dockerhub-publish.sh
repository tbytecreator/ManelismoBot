#!/usr/bin/env bash

set -euo pipefail

DOCKERHUB_USER="${DOCKERHUB_USER:-tbytecreator}"
TAG="${TAG:-latest}"

BACKEND_LOCAL_IMAGE="manelismo-bot-backend:${TAG}"
FRONTEND_LOCAL_IMAGE="manelismo-bot-frontend:${TAG}"

BACKEND_REMOTE_IMAGE="${DOCKERHUB_USER}/manelismo-bot-backend:${TAG}"
FRONTEND_REMOTE_IMAGE="${DOCKERHUB_USER}/manelismo-bot-frontend:${TAG}"

if ! command -v docker >/dev/null 2>&1; then
	echo "Erro: docker nao encontrado no PATH."
	exit 1
fi

echo "Verificando acesso ao daemon Docker..."
if ! docker info >/dev/null 2>&1; then
	echo "Erro: sem acesso ao daemon Docker."
	echo "Dica: inicie o Docker e garanta permissao do usuario no socket."
	exit 1
fi

echo "Build backend (${BACKEND_LOCAL_IMAGE})..."
docker build -f Dockerfile.backend -t "${BACKEND_LOCAL_IMAGE}" .

echo "Build frontend (${FRONTEND_LOCAL_IMAGE})..."
docker build -f Dockerfile.frontend -t "${FRONTEND_LOCAL_IMAGE}" .

echo "Tag backend para Docker Hub (${BACKEND_REMOTE_IMAGE})..."
docker tag "${BACKEND_LOCAL_IMAGE}" "${BACKEND_REMOTE_IMAGE}"

echo "Tag frontend para Docker Hub (${FRONTEND_REMOTE_IMAGE})..."
docker tag "${FRONTEND_LOCAL_IMAGE}" "${FRONTEND_REMOTE_IMAGE}"

echo "Push backend (${BACKEND_REMOTE_IMAGE})..."
if ! docker push "${BACKEND_REMOTE_IMAGE}"; then
	echo "Erro ao publicar backend."
	echo "Se o erro for de autenticacao, execute: docker login"
	exit 1
fi

echo "Push frontend (${FRONTEND_REMOTE_IMAGE})..."
if ! docker push "${FRONTEND_REMOTE_IMAGE}"; then
	echo "Erro ao publicar frontend."
	echo "Se o erro for de autenticacao, execute: docker login"
	exit 1
fi

echo "Publicacao concluida com sucesso."
echo "- ${BACKEND_REMOTE_IMAGE}"
echo "- ${FRONTEND_REMOTE_IMAGE}"