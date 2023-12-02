# Build
## Requirements
1. ***rustup*** (https://www.rust-lang.org/tools/install)
2. ***wasm-pack*** (https://rustwasm.github.io/wasm-pack/installer/)
3. ***npm*** (https://nodejs.org/en/download)
4. ***python*** (https://www.python.org/downloads/)

### Compile Rust lib to wasm
Go into the project root and run

    wasm-pack build

A directory (./pkg) should be created.

### Install packages from packages.json
Go into the www directory and run

    npm install

### create js bundle, that is used by index.html
Go into the www directory and run

    npx webpack -config webpack.conf.js

A directory (./dist), which contains the file bundle.js, should be created.

# Run
## Run a test server on https://localhost:8000
Go into the www directory and run

    python server.py