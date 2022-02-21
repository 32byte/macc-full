Windows:

Requirements: 
- node v17.5.0
- npm v8.4.1
- Install dependencies with `npm install`


Run dev:
- `npm run dev`

Build production:
- `npm run build`

Start production:
- `npm run start`

Linux:

Requirements:
- Rust
    - wasm-pack: `cargo install wasm-pack`
- npm

Steps to run production:

Build production:
- Start dev server first and make sure everything works
- `npm run build`

Start production:
- `npm run start`

Steps to start dev server:
- Build wasm module: `wasm-pack build`
- Install libraries: `npm install`
- Start dev server: `npm run dev`
