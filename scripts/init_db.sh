#!/usr/bin/env bash

# 命令会开启 Bash 的调试模式，会显示出执行的每个命令以及其扩展后的参数，方便调试脚本
set -x

# -e 选项表示在命令执行失败时立即退出脚本
# -o pipefail 选项表示在管道命令中，如果任何一个命令执行失败则整个管道命令都会返回失败
set -eo pipefail

# 检查 psql 命令是否安装
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: `psql` is not installed."
  exit 1
fi

# 检查 sqlx 命令是否安装
if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: `sqlx` is not installed."
  echo >&2 "Use:"
  echo >&2 " cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

# 它的作用是将一个环境变量 DB_USER 的值设置为 POSTGRES_USER 的值，如果 POSTGRES_USER 未定义，则将其设置为 "postgres"。
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

# docker 启动 postgresql 容器
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_PORT}":5432 \
  -d postgres \
  postgres -N 1000
fi

# 检查 postgresql 是否启动
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

# 设置环境变量
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

# 执行数据库创建
sqlx database create

# 执行数据库迁移
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"