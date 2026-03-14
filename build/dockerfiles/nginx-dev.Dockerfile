FROM debian:trixie-slim AS build

WORKDIR /app

RUN <<EOF
set -e
apt-get update
apt-get install -y wget
EOF

RUN <<EOF
set -e
wget https://github.com/swagger-api/swagger-ui/archive/refs/tags/v5.18.2.tar.gz
tar xf v5.18.2.tar.gz
mv swagger-ui-5.18.2/dist swagger-ui
EOF

FROM nginx:alpine AS final

COPY --from=build /app/swagger-ui /usr/share/nginx/html/swagger-ui
COPY ./build/nginx/swagger-initializer.js /usr/share/nginx/html/swagger-ui/
