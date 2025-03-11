# DNSQueryX

🔎 一个基于 Rust + Actix Web 开发的 DNS 查询 API，支持速率限制，并提供标准化 JSON 响应格式。

## ✨ 项目特点

- 🚀 高性能：基于 actix-web 和 tokio 提供异步处理能力。
- 🛡️ 速率限制：集成 actix-governor，防止滥用请求。
- 📡 DNS 解析：使用 trust-dns-resolver 解析域名。
- 📋 标准 API 返回：使用 JSON 格式返回 code/msg/data 结构化数据。
- 📜 日志记录：集成 tracing 和 tracing-subscriber 进行详细日志记录。

## 🚀 使用方法

1. 克隆仓库

    ```bash
    git clone https://github.com/puffinjiang/DNSQueryX.git
    ```

2. 运行项目
    
    确保已安装 Rust（建议使用 rustup）。然后执行：

    ```bash
    cargo run
    ```

默认会启动 http://127.0.0.1:8000。


## 🛠️ API 说明

🔹 1. DNS 查询
- 请求方式：GET
- 接口地址：/dns_lookup
- 请求参数：
- domain（必填）：要解析的域名

示例请求：

```bash
curl http://127.0.0.1:8000/dns_lookup?domain=example.com
```

成功响应示例：

```json
{
  "code": "00000",
  "msg": "OK",
  "data": {
    "domain": "github.com",
    "addresses": ["140.82.112.4"]
  }
}
```

错误响应示例:

```json
{
  "code": "11001",
  "msg": "Missing domain parameter",
  "data": null
}
```

## 📝 TODO

- [ ] 增加 AAAA 记录解析（IPv6）。
- [ ] 添加 Dockerfile，支持容器化部署。



## 📜 许可证

本项目使用 MIT 许可证，你可以自由使用和修改代码。


