FROM rust:1.67

WORKDIR /usr/src/app
COPY . .

WORKDIR /usr/src/app/rym_frontend
RUN apt-get update && apt-get install -y npm
RUN npm i -g npx
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN trunk build --release

WORKDIR /usr/src/app/rym_backend
RUN cargo install --path .
EXPOSE 5001

CMD ["rym_backend"]
