# reversi-bot

**Play "Reversi" with a simple, but reasonably strong bot!**

<img align="right" src="https://user-images.githubusercontent.com/83964523/233449005-5fb8e0bb-45fb-435f-886d-9e6dee3a85ac.png" width="45%" />

- Demo - [Click Here](https://porinky0424.github.io/reversi-bot-front/)
- TS files run wasm functions to calculate bot's thought, which are compiled from Rust code. See [reversi-bot-front](https://github.com/porinky0424/reversi-bot-front) for UI.

## Built with

UI is built with:

- [React](https://ja.reactjs.org/) with [TypeScript](https://www.typescriptlang.org/)
- [Three.js](https://threejs.org/)

bot is built with:

- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)

## To Update

When you update something in `./src` in this repository, you need to run:

```
sh build_and_copy.sh
```

to update `./www/src/pkg`.

And then, you can run in `./www`:

```
yarn; yarn start
```

to start the development server.

## How We Built

By running:

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

we created a new Rust project with wasm-pack template. Here we deleted `.git` and reinitialized it.

At the time we implemented some logics in Rust, we ran:

```
wasm-pack build --target web
```

to compile Rust code into wasm with `build_and_copy.sh`. Then, we copied `./pkg` to `./www/src/pkg`. Now we can use wasm functions in JS. It is a bit tricky, but it works.

## What We Left

- Improve UI (e.g. durable with the change of the window size, button sizes, etc.)
- Use multithreading in Rust

## What We Struggled With

- When we use `rand` crate in wasm, we need to specify the `features` in `Cargo.toml` like:

```
[dependencies]
rand = { version = "0.6.1", features = ["wasm-bindgen"] }
```

- When we do not use jekyll in GitHub Pages, we need to add `.nojekyll` file in root to avoid 404 error.

## Reference

- [Hello, World! - Rust and WebAssembly](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html)
- [Implementing Othello using a bit board](https://qiita.com/sensuikan1973/items/459b3e11d91f3cb37e43) (in Japanese)
