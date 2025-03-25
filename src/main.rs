use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Serialize;
use std::net::IpAddr;
use std::{collections::HashMap, env};
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
    // get the domain from the query
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

    // create the dns resolver
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    // perform the dns lookup
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
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let server_addr = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

    // get rate limit configuration, default to 3 request per second
    let per_second = env::var("RATE_LIMIT_PER_SECOND")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(3);

    // get rate limit burst size, default to 10
    let burst_size = env::var("RATE_LIMIT_BURST_SIZE")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(10);

    // build the governor config
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(per_second.into())
        .burst_size(burst_size)
        .finish()
        .unwrap();

    info!(
        "Starting DNSQueryX server on {} with rate limit: {} req/s, burst size: {}",
        server_addr, per_second, burst_size
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Governor::new(&governor_conf))
            .service(dns_lookup)
    })
    .bind(&server_addr)?
    .run()
    .await
}
