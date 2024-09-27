@[toc]
# esp-rs 简介
esp-rs 是一个专注于为 Espressif 系列芯片（如 ESP32、ESP32-S2、ESP32-C3 等）提供 Rust 语言支持的社区和项目。它的目标是为开发者提供一个高效、安全且易于使用的 Rust 开发环境，以便在 Espressif 芯片上进行嵌入式系统开发。

# Github
- esp-hal 非标准库：[https://github.com/esp-rs/esp-hal](https://github.com/esp-rs/esp-hal)

# Rust 包仓库
- [https://crates.io/](https://crates.io/)

# Wokwi 电子模拟器
- [https://wokwi.com/projects/410182337086340097](https://wokwi.com/projects/410182337086340097)

![在这里插入图片描述](https://i-blog.csdnimg.cn/direct/0b7c59457490453bba2bc363ed048539.png)

# 开发环境
## Rust 环境
- 参考我的这篇文章 [《使用 Rustup 管理 Rust 版本》](https://blog.csdn.net/weixin_42607526/article/details/140048375)

```bash
# 安装 nightly 版本
rustup install nightly
# 设置默认 Rust 版本
rustup default nightly
# 当前 Rust 版本
rustc --version
```

## esp-rs 环境
```bash
rustup component add rust-src --toolchain nightly
# 安装 cargo-generate
cargo install cargo-generate
cargo generate -h
# 安装 espflash
cargo install espflash
espflash --help
espflash flash --help
```

## 创建 ESP32C3 项目
```bash
cargo generate -a esp-rs/esp-template
```

![在这里插入图片描述](https://i-blog.csdnimg.cn/direct/48e1e26afb31449bb73de5a7c0f40d5d.png)

## 编译项目命令
```bash
cd esp-rs-demo
# 默认 debug
cargo build
# 或指定 release
cargo build --release
```

## 运行模拟器
> **注：** VSCode 需要安装 **wokwi** 插件

![在这里插入图片描述](https://i-blog.csdnimg.cn/direct/9a2f6a55588242878cf61c938bc25a42.png)
## ESP32C3 烧录
```bash
espflash flash --monitor target/riscv32imc-unknown-none-elf/debug/esp-rs-demo
```

![在这里插入图片描述](https://i-blog.csdnimg.cn/direct/16294f5426764a80a38d274129e19552.png)


