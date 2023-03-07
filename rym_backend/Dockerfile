FROM rust:1.67

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .
EXPOSE 5001

CMD ["rym_backend"]
