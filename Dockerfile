FROM rust:1.67

WORKDIR /usr/src/app
COPY . .

WORKDIR /usr/src/app/rym_frontend
RUN cargo install trunk
RUN trunk build --release

WORKDIR /usr/src/app/rym_backend
RUN cargo install --path .
EXPOSE 5001

CMD ["rym_backend"]
