# Stage 1: Build the React frontend
FROM node:14 AS frontend-builder

WORKDIR /frontend
COPY client/package*.json ./
RUN npm install
COPY client/ ./
RUN npm run build

# Stage 2: Build the Rust backend
FROM rust:latest AS backend-builder

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8088

WORKDIR /app
COPY . .

RUN cargo build --release

# Stage 3: Create the final image
FROM rust:latest

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8088

WORKDIR /app
COPY --from=backend-builder /app/target/release/rabbitmq-messages-management /app/rabbitmq-messages-management
COPY --from=backend-builder /app/static /app/static

CMD ["./rabbitmq-messages-management"]
