# audio-mixer
Application to learn Rust


## Building

This project uses WebAssembly and it must be pre-built before running the application.

```bash
cargo install wasm-pack
cd wasm-api
wasm-pack build --target web
```

## Development

### WebAssembly API

Stack used: Rust, WebAssembly

#### Install dependencies
```bash
cd wasm-api
cargo install wasm-pack
```


### Web UI

Stack used: React, Vite, Shadcn/ui, TailwindCSS

#### Install dependencies
---
Note: The binding to the WebAssembly API is done through a local link on package.json.
```json
...
"wasm-api": "link:../wasm-api/pkg"
...
```
---
##### Build the WebAssembly API
This must be done before running the web UI.
Make sure you have `wasm-pack` installed.
If not, you can install it with:
```bash
cd wasm-api
wasm-pack build --target web
```
##### Install dependencies
```bash
cd web-ui
pnpm install
```

#### Add a new component
```bash
cd web-ui
pnpm dlx shadcn@latest add button
```