# wasm-pathfinder
 
## Description
This is a demo of pathfinding algorithm in Rust compiled to WebAssembly.
The demo can be found [here](http://example.com).

## How to build
This project uses trunk to build and serve the web app. 
You can install it with `cargo install --locked trunk`.
Then you can run `trunk build --release` to build the project.
The output will be placed in `dist/` directory.

## How to run
Just run `trunk serve`.
If you don't have trunk installed you can install it with `cargo install --locked trunk`.
Then open [localhost:8080](http://localhost:8080) in your browser.

## Resources
- [happycoding.io/pathfinding](https://happycoding.io/tutorials/libgdx/pathfinding)
