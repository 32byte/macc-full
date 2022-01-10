# MACC - Explorer, Wallet, Faucet

This code contains the frontend for a cryptocurrency node.


## Running with docker
Follow the these [instructions](https://github.com/32byte/macc-full/tree/docker)


## Running

1. Build WASM: `npm run build:wasm`
2. Bind WASM: `npm run build:bindgen`
3. Install nodejs dependencies: `npm install`
3.1 Install dev dependencies: `npm install babel-core babel-loader webpack-merge --save-dev`
4. Run frontend: `npm run start`