#!/usr/bin/env bash

set -e

PROJECT_ROOT=$(dirname "$(dirname \""$0"\")")
SPEC="${PROJECT_ROOT}/openapi.json"
GENERATED="${PROJECT_ROOT}/src/api/generated"
CONFIG="${GENERATED}/config.json"
TMP="${PROJECT_ROOT}/tmp"

wget --no-check-certificate "http://webserver:8080/docs/frontend.json" -O "$SPEC"

mkdir -p "${TMP}"
mkdir -p "${GENERATED}/.openapi-generator"
touch "${GENERATED}/config.json" "${GENERATED}/.openapi-generator-ignore" "${GENERATED}/README.md"
mv "${GENERATED}/config.json" "${GENERATED}/.openapi-generator-ignore" "${GENERATED}/.openapi-generator" "${GENERATED}/README.md" "${TMP}"
rm -rf "$GENERATED"
mkdir -p "$GENERATED"
mv "${TMP}/config.json" "${TMP}/.openapi-generator-ignore" "${TMP}/.openapi-generator" "${TMP}/README.md" "${GENERATED}"

npx orval -i "${SPEC}" -o "${GENERATED}" -c "${CONFIG}"

npx prettier --write "${SPEC}"
