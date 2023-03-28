## 快速开始

#### 启动程序
```shell
# 启动数据库
chmod +x scripts/init_db.sh
./scripts/init_db.sh

# 启动程序
cargo run
# 携带日志级别
RUST_LOG=trace cargo run
```

#### 初始化数据库容器
```shell
# 跳过启动容器
SKIP_DOCKER=true ./scripts/init_db.sh
```

#### 构建容器
```shell
docker build -t zero2prod . 
docker run -p 8000:8000 zero2prod
```

#### 构建测试
```shell
# Build test code, without running tests
cargo build --tests
# Find all files with a name starting with `health_check`
ls target/debug/deps | grep health_check

# file descriptor
ulimit -n 10000 && cargo test
```

## curl
```shell
# health check
curl -v http://127.0.0.1:8000/health_check
```

## issues
```text
# tracing-bunyan-formater error
https://github.com/LukeMathWalker/zero-to-production/issues/119
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

# add status
sqlx migrate add add_status_to_subscriptions

# migrate
SKIP_DOCKER=true ./scripts/init_db.sh

# backfikl
sqlx migrate add make_status_not_null_in_subscriptions
```

## 常用命令
```shell
# 删除未使用的依赖
cargo install cargo-udeps
# cargo-udeps requires the nightly compiler.
# We add +nightly to our cargo invocation
# to tell cargo explicitly what toolchain we want to use.
cargo +nightly udeps

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

# 执行测试
cargo test
# 打印 println!
cargo test -- --nocapture
# 指定测试文件
cargo test --test health_check
# 指定测试函数
cargo test fn_name
# 检查测试代码是否编写正确不运行测试
cargo test --no-run
# 重复测试 100 次
repeat 100 { cargo test }
# 指定 package
cargo test --workspace domain
cargo test domain
cargo test valid_emails

# 依赖检查，功能类似于 cargo build，只不过不生成目标文件
cargo check
cargo check --all-targets

# 扩展宏
cargo install cargo-expand(cargo expand)

# 添加 actix-rt  under `[dev-dependencies]` in Cargo.toml
cargo add actix-rt --dev

# 添加 reqwest under `[dev-dependencies]` in Cargo.toml
`cargo add reqwest --dev

# 添加 tokio under `[dev-dependencies]` in Cargo.toml
cargo add tokio --dev

# 添加 config 读取配置文件
cargo add config

# 添加单例模式创建
cargo add once_cell --dev

# We are using the `bunyan` CLI to prettify the outputted logs
# The original `bunyan` requires NPM, but you can install a Rust-port with
# `cargo install bunyan`
TEST_LOG=true cargo test health_check_works | bunyan

# It must be invoked as a cargo subcommand
# All options after `--` are passed to cargo itself
# We need to point it to our binary using --bin
cargo sqlx prepare -- --bin zero2prod

# 增加 serde 辅助函数 string to number
cargo add serde-aux

# unicode valid
cargo add unicode-segmentation

# most assert info
cargo add --dev claim

# email valid
cargo add validator

# retrive body url
cargo add linkify --dev
```

## 集成测试

#### 基本写法
```text
1. 利用 actix-rt 提供的运行时；
2. 资源清理（多次测试是否会出现端口占用）；
3. 并行测试（是否会出现端口占用，导致测试失败）；
4. 如何利用 OS 分配随机端口；
5. 启动服务时不是硬编码，而是传递参数；
完成标准：多次运行测试用例（cargo test），不会出现失败;
```

#### 测试的作用
```text
编写测试确实可以帮助我们构建技术优势，但编写测试需要花费一定的时间。
当我们有紧迫的截止日期时，为了节省时间，往往会牺牲测试工作。也就是说，在时间紧迫的情况下，测试通常是最容易被忽视或削减的工作之一。
然而，这种做法可能会导致质量问题和技术债务的增加，最终可能需要更多的时间来解决这些问题，因此在项目开发中不能忽视测试的重要性。
```

#### 解决测试复杂结构的方法
```text
如果单个测试文件 tests/api/subscriptions.rs 变得过于复杂和混乱，我们可以将其转换为一个模块，并且创建一个名为 tests/api/subscriptions/helpers.rs 的文件来保存与订阅测试相关的帮助函数。
通过这种方式，我们可以将不同的测试用例组织到多个专注于特定流程或关注点的测试文件中。这些测试文件可以在 tests/api/subscriptions/ 目录下，每个文件只关注于一个特定的功能或方面。
这样做的好处是，它可以使测试代码更加清晰、易于维护和扩展。同时，我们还可以将测试辅助函数的实现细节封装在 helpers.rs 文件中，从而进一步提高代码的可读性和可维护性。
```

#### 测试编译优化
当你编写一个 Rust 项目时，通常会创建一个 src 目录用于放置源代码文件，以及一个 tests 目录用于放置测试用例。如果你的测试用例非常多且文件结构很扁平，比如：
```text
tests/
├── test1.rs
├── test2.rs
├── test3.rs
├── test4.rs
├── test5.rs
├── test6.rs
├── test7.rs
├── test8.rs
├── test9.rs
└── test10.rs
```
那么每次运行 cargo test 命令时，Cargo 将会依次编译和链接这些测试用例，这意味着编译时间和链接时间将呈现线性增长。

但是，如果你将所有测试用例放在一个单独的文件中，比如 tests/all.rs：
```text
// tests/all.rs
mod test1;
mod test2;
mod test3;
mod test4;
mod test5;
mod test6;
mod test7;
mod test8;
mod test9;
mod test10;
```
那么每次运行 cargo test 命令时，Cargo 只需要编译和链接这个文件一次，即可执行所有测试用例。这样可以大大减少编译和链接时间，提高构建速度。

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

# sql injection
https://en.wikipedia.org/wiki/SQL_injection

# active-web log middleware
https://docs.rs/actix-web/4.0.0-beta.1/actix_web/middleware/struct.Logger.html

# mio
https://docs.rs/mio/latest/mio/

# tracing subscriber
https://docs.rs/tracing/0.1.19/tracing/trait.Subscriber.html

# 0.0.0.0
https://github.com/sinatra/sinatra/issues/1369

# symbols from binary
https://github.com/johnthagen/min-sized-rust#strip-symbols-from-binary

# rust-musl-builder
https://github.com/emk/rust-musl-builder

# rust 年度调差 2019
https://blog.rust-lang.org/2020/04/17/Rust-survey-2019.html#rust-adoption---a-closer-look

# actix-web panic
https://github.com/actix/actix-web/issues/1501

# 随机生成测试数据
https://crates.io/crates/quickcheck
https://crates.io/crates/proptest

# rust wiremock
https://github.com/lukemathwalker/wiremock-rs
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