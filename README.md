## 快速开始
```shell
# 启动程序
cargo run
```

## 书籍简介
```text
https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/
```

## 常用命令
```shell
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
1. 无状态应用
平时写的 web 服务进行数据持久化的时候不会依赖于机器本地的文件系统，而会使用外部系统进行数据存储；（这种叫无状态应用）
```