# start-rust

一个 Rust 跨平台脚手架项目，使用 C/C++ vcpkg 集成搭建


# `Rust`

官网：https://www.rust-lang.org/zh-CN/

仓库：https://github.com/rust-lang

平台支持:
- `Tier 1` 级支持主要是 x86 Desktop
- `Tier 2` 级支持包含了 ARM64 macOS, iOS 和 Android
- `Tier 3` 级不会自动构建或测试

| Tier 1 target             | notes                                   |
| ------------------------- | --------------------------------------- |
| aarch64-unknown-linux-gnu	| ARM64 Linux (kernel 4.1, glibc 2.17+)   |
| i686-pc-windows-gnu	    | 32-bit MinGW (Windows 7+)               |
| i686-pc-windows-msvc	    | 32-bit MSVC (Windows 7+)                |
| i686-unknown-linux-gnu	| 32-bit Linux (kernel 3.2+, glibc 2.17+) |
| x86_64-apple-darwin	    | 64-bit macOS (10.12+, Sierra+)          |
| x86_64-pc-windows-gnu	    | 64-bit MinGW (Windows 7+)               |
| x86_64-pc-windows-msvc    | 64-bit MSVC (Windows 7+)                |
| x86_64-unknown-linux-gnu	| 64-bit Linux (kernel 3.2+, glibc 2.17+) |


## 1、`Rustup`：`Rust` 安装器和版本管理工具

安装 `Rust` 的主要方式是通过 `Rustup` 这一工具，它既是一个 `Rust` 安装器又是一个版本管理工具。

> 也可以选择其他安装方式：https://forge.rust-lang.org/infra/other-installation-methods.html

```bash
# On Unix: 下载并运行 rustup-init.sh 在线安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# On Windows: 下载并运行 rustup-init.exe 在线安装
https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe

# 选择 1 默认安装
# 安装完成需要 source 或重启，否则无法识别 rustc

rustc --version      # 版本: rustc 1.74.1 (a28077b28 2023-12-04)
rustup update        # 更新 Rust
```


## 2、`Cargo`：`Rust` 的构建工具和包管理器

安装 `Rustup` 时也会安装 `Rust` 构建工具和包管理器的最新稳定版，即 `Cargo`。

```bash
cargo --version      # 版本: cargo 1.74.1 (ecb9851af 2023-10-18)
cargo new start-rust # 创建新项目
cargo build          # 构建项目 --release
cargo run            # 运行项目
cargo test           # 单元测试
cargo doc            # 为项目构建文档
cargo publish        # 将库发布到 crates.io
cargo add xxx        # 添加依赖库 Cargo.toml
```

配置文件：`Cargo.toml`

包仓库：https://crates.io/

### `crates`：包

`crate` 分为两种：`binary crate` and `library crate`，一个包可以包含任意多个 `binary crate`，但最多只能包含一个 `library crate`。

- `src/main.rs` 和 `src/lib.rs` 是根文件，编译器从这里开始查找。
- `src/lib/mod.rs` 旧的写法


## 3、编译器 & 调试器

VS Code 安装 `rust-analyzer` 分析器插件。

### Windows
- 依赖 `Microsoft C++ Build Tools` 并选择 `使用 C++ 进行桌面开发`。
- VS Code 安装 `C/C++` 调试器插件。

### macOS
- 依赖 `Xcode`，可以通过 `xcode-select --install` 安装。
- VS Code 安装 `CodeLLDB` 调试器插件。

### Linux
- 依赖 `GCC`，可以通过 `sudo apt-get install build-essential` 安装。
- VS Code 安装 `CodeLLDB` 调试器插件。
