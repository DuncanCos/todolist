# ÉTAPE 1 : Le Builder
FROM rust:latest as builder

WORKDIR /usr/src/app

# 1. On crée un projet vide pour mettre en cache les dépendances
RUN USER=root cargo new --bin todoback
WORKDIR /usr/src/app/todoback

# 2. On copie uniquement les manifestes de dépendances
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. On compile uniquement les dépendances (cela sera mis en cache par Docker)
RUN cargo build --release
RUN rm src/*.rs

# 4. Maintenant, on copie votre vrai code source
COPY ./src ./src

# 5. On compile votre application (seule cette étape sera refaite si le code change)
# On utilise 'touch' pour forcer Cargo à voir le changement de fichier
RUN rm ./target/release/deps/todoback*
RUN cargo build --release

# ÉTAPE 2 : Le Runner
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/todoback/target/release/todoback /usr/local/bin/app

EXPOSE 8080
CMD ["app"]