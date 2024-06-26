# 🛠 Rgine [![Documentation][doc-img]][doc-url]

[doc-img]: https://img.shields.io/badge/docs.rs-rgine-4d76ae
[doc-url]: https://Swiiz.github.io/rgine

**🦀 Rust modular game engine made from scratch.
The code is divided into crates, each having example to better understand how the engine works.**

Check out the todo list at `todo.txt`.

## Examples and how to run

> Event scheduling debug logging can be enabled using the flag `--features "rgine/schedulelog"` or the `schedulelog` feature of the rgine root crate. (Consider logging into a file using for example `> log.txt` on windows)

### Main examples:

- **2D Rendering:**  
Path: `examples/render2d`  
How to run: `cargo run -p rgine_render2d_example`

### Core modules examples:

> Event scheduling debug logging can be enabled using the flag `--features "rgine_modules/debuglog"` or the `debuglog` feature of the crate. (Consider logging into a file using for example `> log.txt` on windows)

#### Graphics context:

- **Simple Render pass:**  
Path: `core/graphics/examples/simple.rs`  
How to run: `cargo run -p rgine_graphics --example simple`

#### Platform:

- **Windowed:**  
Path: `core/platform/examples/windowed.rs`  
How to run: `cargo run -p rgine_platform --example windowed`

- **Headless:** TODO...

#### Modules:

- **Walkthrough:**  
Path: `core/modules/examples/walkthrough.rs`  
How to run: `cargo run -p rgine_modules --example walkthrough`





