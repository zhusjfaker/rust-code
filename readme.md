# Rust 与 C 与 Javascript 参考示例

## 文件结构

```sh
.
├── Cargo.lock
├── Cargo.toml
├── poplar           // 调用 rust-ffmpeg 测试示例库 实现一个简单的 ffmpeg 截图功能
├── readme.md
├── rust-ffmpeg      // ffmpeg 跨平台 如果编译成 被 rust 可以调用 rslib 包括 (自动生成 rust 定义库 类似 *.d.ts 的功能利用 bindgen)
├── target
└── wasm-test        // 实现一个 wasm-pack (rust 构建 wasm 工具) 打包用例 其中包含 c 语言实现的静态库 *.a 的 3平台 FFI 交互操作 
```


## 命令介绍

* 包依赖恢复
```
cargo update
```

* Monorepo 编译
```
cargo build
```

* 单包 编译
```
cargo build --name {包名称}
```

* 测试用例运行
```
cargo test {lib.rs 中 函数名} -- --nocapture
```
