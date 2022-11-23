# Yew Web3 Template

This is a template for using Web3 browser extensions (like Metamask, etc) from yew, using rust and webassembly.

An example of the code this project generates can be found [here](https://monomadic.github.io/yew-web3-template-example/).

## Installing with `cargo-generate` (recommended)
```bash
cargo install cargo-generate
cargo generate --git https://github.com/monomadic/yew-web3-template
```

## Installing with `git`
```bash
git clone --depth=1 https://github.com/monomadic/yew-web3-template
rm -rf .git && git init
```

## Running

### `cargo-make` (recommended)
```bash
cargo install cargo-make
cp .env.local.example .env.local
cargo make serve
```

### without `cargo-make`
```bash
cp .env.local.example .env.local
trunk serve
```
