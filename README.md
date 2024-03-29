# reversi-bot

**Play "Reversi" with a simple, but reasonably strong bot!**

<img src="https://user-images.githubusercontent.com/83964523/233449005-5fb8e0bb-45fb-435f-886d-9e6dee3a85ac.png" width="50%" />

### DEMO - [Click here](https://porink0424.github.io/reversi-bot-front/)

- TS files run wasm functions to calculate bot's thought, which are compiled from Rust code. See [reversi-bot-front](https://github.com/porink0424/reversi-bot-front) for UI.</li>
- When getting closer to the end of the game, the bot will read all the possible moves, choose the best one, and give you a prediction of the result, which is definitely correct if the prediction says you will lose. You have to give up 🥲

## Built with

UI is built with:

- [React](https://ja.reactjs.org/) with [TypeScript](https://www.typescriptlang.org/)
- [Three.js](https://threejs.org/)

bot is built with:

- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)

## Getting Started

```
git clone git@github.com:porink0424/reversi-bot.git
cd reversi-bot
git clone git@github.com:porink0424/reversi-bot-front.git www
```

When you update something in `/src` in this repository, you need to run:

```
sh build_and_copy.sh
```

to update `/www/src/pkg`.

And then, you can run in `/www`:

```
yarn; yarn start
```

to start the development server.

## How I Built

By running:

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

I created a new Rust project with wasm-pack template. Here I deleted `.git` and reinitialized it.

At the time I implemented some logics in Rust, I ran:

```
wasm-pack build --target web
```

to compile Rust code into wasm with `build_and_copy.sh`. Then, I copied `./pkg` to `./www/src/pkg`. Now I can use wasm functions in JS. It is a bit tricky, but it works.

## What I Left

- Improve UI (e.g. durable with the change of the window size, button sizes, etc.)
- Use multithreading in Rust
  - We cannon use `std::thread` in wasm, so we need to use `web_sys::Worker` or something like that...

## What I Struggled With

- When we use `rand` crate in wasm, we need to specify the `features` in `Cargo.toml` like:

```
[dependencies]
rand = { version = "0.6.1", features = ["wasm-bindgen"] }
```

- When we do not use jekyll in GitHub Pages, we need to add `.nojekyll` file in root to avoid 404 error.

## Reference

- [Hello, World! - Rust and WebAssembly](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html)
- [Implementing Othello using a bit board](https://qiita.com/sensuikan1973/items/459b3e11d91f3cb37e43) (in Japanese)
