#!/usr/bin/env bash

set -e

mkdir -p data/ca-certificates data/conf/nginx-dev

CA_KEY="/tmp/matrix-ca.key"
CA_CRT="data/ca-certificates/ca-dev.crt"

NGINX_KEY="data/conf/nginx-dev/matrix.key"
NGINX_CRT="data/conf/nginx-dev/matrix.crt"
NGINX_CSR="/tmp/matrix-dev.csr"

openssl \
    req -new -x509 \
    -newkey rsa:4096 \
    -keyout $CA_KEY \
    -nodes \
    -out $CA_CRT \
    -subj '/CN=MatrixTicketSystem CA/O=DevCerts' \
    -addext 'subjectKeyIdentifier = hash' \
    -addext 'basicConstraints = critical,CA:true' \
    -addext 'keyUsage = critical, digitalSignature, cRLSign, keyCertSign' \
    -days 365

openssl req -new \
    -newkey rsa:4096 \
    -keyout $NGINX_KEY \
    -nodes \
    -out $NGINX_CSR \
    -subj '/CN=MatrixTicketSystem Server/O=DevCerts' \
    -addext "subjectAltName = DNS:localhost, DNS:nginx"

# 2. Sign the CSR with your Local CA
openssl x509 -req \
    -in $NGINX_CSR \
    -CA "$CA_CRT" \
    -CAkey "$CA_KEY" \
    -CAcreateserial \
    -out "$NGINX_CRT" \
    -days 365