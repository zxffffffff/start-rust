# start-rust

一个 Rust 跨平台脚手架项目，使用 C/C++ vcpkg 集成搭建

# `Rust`

官网：https://www.rust-lang.org/zh-CN/

仓库：https://github.com/rust-lang

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
cargo build          # 构建项目
cargo run            # 运行项目
cargo test           # 测试项目
cargo doc            # 为项目构建文档
cargo publish        # 将库发布到 crates.io。
```

## 3、`crates`：包

包依赖文件：`Cargo.toml`

包仓库：https://crates.io/

## 4、编译器

`VS Code` 使用 `rust-analyzer` 插件。

注意：需要安装带有 `C/C++` 链接器的工具集来组合 `Rust` 编译器输出。

### Windows

依赖 `Microsoft C++ Build Tools` 并选择 “使用 C++ 进行桌面开发”。

### macOS

依赖 `Xcode`，可以通过 `xcode-select --install` 安装。

### Linux

依赖 `GCC`，可以通过 `sudo apt-get install build-essential` 安装。
