<div align="center">

  <h1><code>Collatz Conjecture Visualization</code></h1>

  <strong>Rust and WebAssembly project that renders a sequence of integers inside HTML canvas element. Integers are obtained by certain arithmetic operations defined by Collatz conjecture.</strong>

</div>

## Preview

![](https://media.giphy.com/media/Po6zT5rKuNGt4tlT94/giphy.gif)

## About

Collatz conjecture, also known as 3x + 1 problem, is an unsolved problem in mathematics and is named after Lothar Collatz who proposed it in one of his papers in 1937.
Begin with any positive integer and generate a sequence as follows: If a
number is even, divide it by two. Else, multiply by three and add one. Repetition of this process will eventually reach the value 1. Proof or disproof of this seemingly simple conjecture has remained elusive.

## Usage

### Build .wasm file with `wasm-pack build`

```
wasm-pack build
```

### Install dependencies with `npm install`

```
npm install
```

### Build the example locally with `npm run serve`

```
npm run serve
```