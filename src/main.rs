use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::collections::HashMap;
use std::net::IpAddr;
use tracing::{error, info};
use tracing_subscriber;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

#[derive(Serialize)]
struct DnsData {
    domain: String,
    addresses: Vec<IpAddr>,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    code: String,
    msg: String,
    data: Option<T>,
}

#[get("/dns-lookup")]
async fn dns_lookup(query: web::Query<HashMap<String, String>>) -> impl Responder {
    // 获取 URL 查询参数中的 "domain"
    let domain = match query.get("domain") {
        Some(d) => d.clone(),
        None => {
            error!("Missing domain parameter");
            return HttpResponse::BadRequest().json(ApiResponse::<DnsData> {
                code: "11001".to_string(),
                msg: "Missing domain parameter".to_string(),
                data: None,
            });
        }
    };

    info!("Received DNS lookup request for domain: {}", domain);

    // 创建 DNS 解析器
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    // 执行 DNS 查询
    match resolver.lookup_ip(&domain).await {
        Ok(response) => {
            let addresses: Vec<IpAddr> = response.iter().collect();
            info!(
                "DNS lookup successful for domain: {}: {:?}",
                domain, addresses
            );
            HttpResponse::Ok().json(ApiResponse {
                code: "00000".to_string(),
                msg: "OK".to_string(),
                data: Some(DnsData { domain, addresses }),
            })
        }
        Err(err) => {
            error!("DNS lookup failed for domain: {}: {}", domain, err);
            HttpResponse::build(actix_web::http::StatusCode::BAD_GATEWAY).json(ApiResponse::<
                DnsData,
            > {
                code: "11002".to_string(),
                msg: err.to_string(),
                data: None,
            })
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化 tracing 日志订阅器
    tracing_subscriber::fmt::init();

    // 配置速率限制：每秒允许 1 次请求，允许最多 5 次突发请求
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(5)
        .finish()
        .unwrap();

    info!("Starting DNSQueryX server on 0.0.0.0:8000");

    HttpServer::new(move || {
        App::new()
            .wrap(Governor::new(&governor_conf)) // 速率限制中间件（超限时自动返回 429）
            .service(dns_lookup)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
