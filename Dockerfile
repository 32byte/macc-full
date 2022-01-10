FROM ubuntu

EXPOSE 8080

ENV DEBIAN_FRONTEND=noninteractive

# update system
RUN apt-get -y update && apt-get install -y
# install packages
RUN apt -y install curl
RUN apt -y install libssl-dev
RUN apt -y install clang
RUN apt -y install build-essential
RUN apt -y install pkg-config
RUN apt -y install git

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
RUN chmod +x rustup.sh
RUN ./rustup.sh -y

ENV PATH="/root/.cargo/bin:${PATH}"

# install nodejs and npm
ENV NODE_VERSION=12.18.1
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
ENV NVM_DIR=/root/.nvm
RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION}

ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"

# update npm
RUN npm install -g npm

# install utilities for wasm
RUN rustup target add wasm32-unknown-unknown --toolchain stable

RUN cargo install wasm-pack
RUN cargo install cargo-generate
RUN cargo install -f wasm-bindgen-cli

# clone
WORKDIR /usr/
RUN git clone https://github.com/32byte/macc-full.git
WORKDIR /usr/macc-full/www/
RUN rustup target add wasm32-unknown-unknown

ARG CACHEBUST=1
RUN git pull

# build
RUN cargo build --target wasm32-unknown-unknown --release

RUN npm run build:wasm
RUN npm run build:bindgen
RUN npm install
RUN npm install babel-core babel-loader webpack-merge --save-dev
RUN npm run build:webpack

CMD ["npm", "run", "start"]