# Yew Web3 Template

This is a template for using Web3 browser extensions (like Metamask, etc) from yew, using rust and webassembly.

An example of the code this project generates can be found [here](https://monomadic.github.io/yew-web3-template-example/).

## Installing with `cargo-generate` (recommended)
```bash
git clone --depth=1 https://github.com/monomadic/yew-web3-template
cp .env.local.example .env.local
cargo install cargo-generate
cargo generate --git https://github.com/monomadic/yew-web3-template
```

## Installing with `git`
```bash
git clone --depth=1 https://github.com/monomadic/yew-web3-template
cp .env.local.example .env.local
cargo install cargo-make
cargo make serve
```
