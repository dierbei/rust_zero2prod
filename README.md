## 快速开始
```shell
# 启动数据库
chmod +x scripts/init_db.sh
./scripts/init_db.sh

# 启动程序
cargo run
```
```shell
# 跳过启动容器
SKIP_DOCKER=true ./scripts/init_db.sh
```

## curl
```shell
# health check
curl -v http://127.0.0.1:8000/health_check
```

## actix-web
```shell
# HttpResponse
https://docs.rs/actix-web/4.0.0-beta.3/actix_web/struct.HttpResponse.html

# HttpResponseBuilder
https://docs.rs/actix-web/4.0.0-beta.3/actix_web/dev/struct.HttpResponseBuilder.html
```

## sqlx-cli
```shell
# 安装
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres

# Assuming you used the default parameters to launch Postgres in Docker!
export DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter
sqlx migrate add create_subscriptions_table
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
# 打印 println!
cargo test-- --nocapture
# 指定测试文件
cargo test --test health_check
# 指定测试函数
cargo test fn_name
# 检查测试代码是否编写正确不运行测试
cargo test --no-run

# 依赖检查，功能类似于 cargo build，只不过不生成目标文件
cargo check

# 扩展宏
cargo install cargo-expand(cargo expand)

# 添加 actix-rt  under `[dev-dependencies]` in Cargo.toml
cargo add actix-rt --dev

# 添加 reqwest under `[dev-dependencies]` in Cargo.toml
`cargo add reqwest --dev

# 添加 tokio under `[dev-dependencies]` in Cargo.toml
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

# rust test options
https://doc.rust-lang.org/book/ch11-03-test-organization.html

# rust target
https://doc.rust-lang.org/cargo/reference/cargo-targets.html#cargo-targets

# unix port 0
https://www.lifewire.com/port-0-in-tcp-and-udp-818145

# tcp listener
https://www.lifewire.com/port-0-in-tcp-and-udp-818145

# post form available data options
https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST

# table driven test
https://github.com/golang/go/wiki/TableDrivenTests

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

# generic type
https://doc.rust-lang.org/book/ch10-01-syntax.html

# Understanding Serde
https://www.joshmcguigan.com/blog/understanding-serde/
```

## concept
```text
1. we expect Cloud-native applications
To achieve high-availability while running in fault-prone environments;
To allow us to continuously release new versions with zero downtime;
To handle dynamic workloads (e.g. request volumes).

2. black box test
This is often referred to as black box testing: we verify the behaviour of a system by examining its
output given a set of inputs without having access to the details of its internal implementation.

3. 全黑盒解决方案是什么意思？
全黑盒解决方案是指一种解决问题的方法，其中包含的细节和实现方式是未知的，只知道输入和输出的关系。
这种解决方案通常基于黑盒测试的原理，即在不知道内部实现细节的情况下，仅通过输入和输出来测试系统或软件的正确性。
在信息技术领域，全黑盒解决方案通常用于描述一些外部服务或系统的接口，例如API（应用程序编程接口）或云服务。
这些接口向用户公开了输入和输出的格式和类型，但没有透露底层实现的细节。
全黑盒解决方案的优点是可以隐藏底层实现的细节，从而降低攻击者对系统的攻击风险。
另外，这种解决方案也可以促进不同团队之间的协作，因为它们不需要知道对方的具体实现细节，只需要遵循公开的接口规范。
缺点是在出现问题时，可能需要更长时间来定位问题的根本原因。
```