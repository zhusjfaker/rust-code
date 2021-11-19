#!/bin/zsh

## rust 编译 wasm 需要依赖 wasm-pack
## 具体参考地址 https://rustwasm.github.io/wasm-pack/installer/
## 具体命令行文档 https://rustwasm.github.io/wasm-pack/book/

wasm-pack build --out-dir out