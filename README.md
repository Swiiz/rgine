# 🛠 Rgine

**🦀 Rust modular game engine made from scratch.
The code is divided into crates, each having example to better understand how the engine works.**

## Examples and how to run

> Event scheduling debug logging can be enabled using the flag `--features "rgine_modules/debuglog"` or the `debuglog` feature of the crate. (Consider logging into a file using for example `> log.txt` on windows)

### Graphics context:

- **Simple Render pass:**  
Path: `core/graphics/examples/simple.rs`  
How to run: `cargo run -p rgine_graphics --example simple`

### Platform:

- **Windowed:**  
Path: `core/platform/examples/windowed.rs`  
How to run: `cargo run -p rgine_platform --example windowed`

- **Headless:** TODO...

### Modules:

- **Walkthrough:**  
Path: `core/modules/examples/walkthrough.rs`  
How to run: `cargo run -p rgine_modules --example walkthrough`





