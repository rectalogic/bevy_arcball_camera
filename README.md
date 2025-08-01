# Bevy Arcball Camera Controller

[Arcball](https://graphicsinterface.org/wp-content/uploads/gi1992-18.pdf)
camera controller for [Bevy](https://bevy.org).
Supports mouse and touch controls.

Drag to rotate camera around a virtual sphere,
mouse wheel or pinch to zoom.

```sh-session
$ cargo run --example demo
```

Web demo:
```sh-session
$ cargo install wasm-bindgen-cli
$ cargo build --release --target wasm32-unknown-unknown --example demo
$ wasm-bindgen --out-dir web/pkg --out-name demo --target web target/wasm32-unknown-unknown/release/examples/demo.wasm

$ python3 -m http.server -d web
```
