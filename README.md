# Solwasmc

> Web assembly compiler for the solidity language.

## Table of contents

- [General info](#general-info)
- [Usage](#usage)
- [Building From Source](#building-from-source)
- [Technologies](#technologies)

## General info

The motivation behind Solcwasmc is to build a solidity compiler that can run in the browser via [web assembly](https://webassembly.org/). This is not meant to replace the canonical [solc](https://github.com/ethereum/solidity.git) in terms of usability, but simply act as a drop in replacement for deploying smart contracts in the browser.

## Usage

`npm i @drdgvhbh/solwasmc`

```ts
import { compile } from "@drdgvhbh/solwasmc";

const byteCode = compile("contract Test {...}");
```

## Building from Source

### Prerequisites

- [Python3](https://www.python.org/downloads/)
- [GNU Make](https://www.gnu.org/software/make/)
- [Cargo](https://github.com/rust-lang/cargo)
- [Rust](https://www.rust-lang.org/)

### Build

```sh
make generate
cargo build
```

### Build for Web Assembly

```sh
wasm-pack build --scope YOUR_NPM_USERNAME
cd pkg
npm publish --access=public
```

## Technologies

- [Nom 5.0.0](https://docs.rs/nom/5.0.0/nom/)
- [Rust](https://www.rust-lang.org/)
