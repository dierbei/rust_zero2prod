## 快速开始
```shell
# 启动程序
cargo run
```

## 常用命令
```shell
# 安装 rust nightly 版本
rustup toolchain install nightly --allow-downgrade

# 更新 rust toolchain
rustup update

# 查看 rust toolchain
rustup toolchain list

# rust 版本 & cargo 版本
rustc --version & cargo --version

# rust code coverage tool
# At the time of writing tarpaulin only supports
# x86_64 CPU architectures running Linux.
cargo install cargo-tarpaulin(cargo tarpaulin --ignore-tests)

# rust lint 静态代码分析
rustup component add clippy(cargo clippy -- -D warnings)

# rust fmt 格式化代码
rustup component add rustfmt(cargo fmt -- --check)

# rust 依赖漏洞检查
cargo install cargo-audit(cargo audit)

# rust 添加 cargo add
cargo install cargo-edit

# 单元测试
cargo test

# 依赖检查，功能类似于 cargo build，只不过不生成目标文件
cargo check

# cargo add 命令前置安装
cargo install cargo-edit

# 扩展宏
cargo install cargo-expand
cargo expand

# 添加 actix-rt
cargo add actix-rt --dev

# 添加 reqwest
`cargo add reqwest --dev

# 展开测试文件
cargo expand --test health_check

# 添加 tokio
cargo add tokio --dev
```

## 集成测试
```text
1. 利用 actix-rt 提供的运行时；
2. 资源清理（多次测试是否会出现端口占用）；
3. 并行测试（是否会出现端口占用，导致测试失败）；
4. 如何利用 OS 分配随机端口；
5. 启动服务时不是硬编码，而是传递参数；
完成标准：多次运行测试用例（cargo test），不会出现失败;
```

## 文档
```text
# Domain Driven Design
https://www.youtube.com/watch?v=PLFl95c-IiU

# type-driven design
https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/

# Test coverage
https://martinfowler.com/bliki/TestCoverage.html

# tarpaulin
https://github.com/xd009642/tarpaulin

# clippy
https://github.com/rust-lang/rust-clippy#configuration

# githup workflow
https://docs.github.com/zh/actions/quickstart

# mdn web post request encode doc
https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST

# url encode doc
https://www.w3schools.com/tags/ref_urlencode.ASP

# actix-web data extraction
https://actix.rs/docs/extractors/

# actix-web path
https://docs.rs/actix-web/4.0.0-beta.3/actix_web/web/struct.Path.html

# actix-web query
https://docs.rs/actix-web/4.0.0-beta.3/actix_web/web/struct.Query.html

# actix-web json
https://docs.rs/actix-web/4.0.0-beta.3/actix_web/web/struct.Json.html

# actix-web form
https://docs.rs/actix-web/4.0.0-beta.3/actix_web/web/struct.Form.html

# serde
https://serde.rs/
```

## 概念理解
```text
1. we expect Cloud-native applications
To achieve high-availability while running in fault-prone environments;
To allow us to continuously release new versions with zero downtime;
To handle dynamic workloads (e.g. request volumes).

```