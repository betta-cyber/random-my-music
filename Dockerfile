FROM rust:1.67

WORKDIR /usr/src/app
COPY . .

WORKDIR /usr/src/app/frontend
RUN curl -sL https://deb.nodesource.com/setup_18.x -o nodesource_setup.sh
RUN bash nodesource_setup.sh
RUN apt-get install -y nodejs
RUN npm install -g npm@9.6.1
RUN npx tailwindcss -i ./styles/tailwind.css -o ../dist/.stage/index.css
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN trunk build --release

WORKDIR /usr/src/app/backend
RUN cargo install --path .
EXPOSE 5001

CMD ["RUN_MODE=production backend"]
