FROM rust:latest AS builder

# 设置工作目录
WORKDIR /app

# 复制 Cargo 配置文件（优化构建缓存）
COPY Cargo.toml Cargo.lock ./

# 复制 src 目录
COPY src src

# 构建可执行文件并显示详细输出
RUN cargo build --release

# 运行阶段，使用更小的基础镜像（Debian Slim）
FROM debian:bookworm-slim

# 设置工作目录
WORKDIR /app

# 复制可执行文件
COPY --from=builder /app/target/release/DNSQueryX /app/dns_queryx

# 暴露端口
EXPOSE 8000

# 运行应用
CMD ["/app/dns_queryx"]