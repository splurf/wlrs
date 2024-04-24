# wlrs

- Minecraft Authenticated Whitelister

## Build
```bash
cargo build --release
trunk build --release
```

## Usage
```bash
cargo run --release    # backend
trunk serve --release  # fronend
```

### Todo
- Further reduce size of WASM binary
    - Maybe use `wasm-opt`
- Implement authentication popup window.
    - Something like [this](https://developer.mozilla.org/en-US/docs/Web/API/Web_Authentication_API) maybe