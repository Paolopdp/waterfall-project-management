FROM rust:1.75-slim

WORKDIR /app

# Installa le dipendenze di sistema necessarie
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev postgresql-client && \
    rm -rf /var/lib/apt/lists/*

# Copia solo i file di Cargo prima per sfruttare la cache delle dipendenze
COPY Cargo.toml Cargo.lock ./

# Crea un progetto fittizio per scaricare le dipendenze
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build && \
    rm -rf src

# Copia il codice sorgente
COPY . .

# Compila in modalità release
RUN cargo build

# Script per eseguire le migrazioni e avviare l'applicazione
COPY ./docker-entrypoint.sh /
RUN chmod +x /docker-entrypoint.sh

EXPOSE 3001

ENTRYPOINT ["/docker-entrypoint.sh"]
