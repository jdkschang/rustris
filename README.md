# Rustris
Tetris implementation in Rust, WASM, and React


#### Inspired by: 
- [yishn's Let's Code](https://github.com/yishn/lets-code/tree/main/tetris) series
- [_youtube walkthrough_][video]

[video]: https://www.youtube.com/watch?v=_lAr7JveRVE

## Building

Make sure you have [Rust](https://www.rust-lang.org) installed and
[wasm-pack](https://rustwasm.github.io/wasm-pack/). To build this project, run:

```
$ wasm-pack build --target web
```

To run this project, you need a static file server. You can install `sfz` with
cargo:

```
$ cargo install sfz
```

Now, start your static file server and open `index.html`:

```
$ sfz -r
```
