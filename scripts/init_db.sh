#!/usr/bin/env bash

# 命令会开启 Bash 的调试模式，会显示出执行的每个命令以及其扩展后的参数，方便调试脚本
set -x

# -e 选项表示在命令执行失败时立即退出脚本
# -o pipefail 选项表示在管道命令中，如果任何一个命令执行失败则整个管道命令都会返回失败
set -eo pipefail

# 它的作用是将一个环境变量 DB_USER 的值设置为 POSTGRES_USER 的值，如果 POSTGRES_USER 未定义，则将其设置为 "postgres"。
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

docker run \
-e POSTGRES_USER=${DB_USER} \
-e POSTGRES_PASSWORD=${DB_PASSWORD} \
-e POSTGRES_DB=${DB_NAME} \
-p "${DB_PORT}":5432 \
-d postgres \
postgres -N 1000
# ^ Increased maximum number of connections for testing purposes