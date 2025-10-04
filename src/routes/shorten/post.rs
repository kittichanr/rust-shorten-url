use actix_web::{HttpRequest, HttpResponse, Responder, web};
use redis::{Client, Commands};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    configuration::Settings,
    helpers::{base62_encode, enforce_http, remove_domain_error},
};

#[derive(Deserialize, Serialize)]
pub struct Request {
    url: String,
    custom_short_url: Option<String>,
    expire_at: Option<i32>,
}

#[derive(Serialize)]
pub struct Response {
    url: String,
    custom_short_url: String,
    expiry: i32,
    x_rate_remaining: i32,
    x_rate_limit_reset: i32,
}

pub async fn post_shorten(
    body: web::Json<Request>,
    redis_pool: web::Data<r2d2::Pool<Client>>,
    req: HttpRequest,
    configuration: web::Data<Settings>,
) -> impl Responder {
    let mut redis_conn = redis_pool
        .get()
        .expect("Failed to get Redis connection from pool");

    let ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();

    let key = format!("ratelimit:{}", ip);

    let val: Option<String> = redis_conn
        .get(&key)
        .expect("failed to execute GET for 'ip address'");

    let limit: i64 = redis_conn
        .ttl(&key)
        .expect("failed to execute TTL for 'ip address'");

    match val {
        None => {
            let quota = configuration.application.api_quota;
            let _: () = redis_conn
                .set_ex(&key, quota, 30 * 60)
                .expect("Failed to set quota");
        }
        Some(val_str) => {
            let val_int: i32 = val_str.parse().unwrap_or(0);

            if val_int <= 0 {
                return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                    "error": "Rate limit exceeded",
                    "rate_limit_reset": limit / 60,
                }));
            }
        }
    }

    if Url::parse(&body.url).is_err() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid URL"
        }));
    }

    if !remove_domain_error(&req.uri().to_string(), &configuration.application.domain) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Can't do that :)"
        }));
    }

    let enforced_url = enforce_http(&body.url);

    let id = if let Some(custom_short) = &body.custom_short_url {
        custom_short.clone()
    } else {
        base62_encode(rand::random::<u64>())
    };

    // Check if custom short URL already exists
    let existing_val: Option<String> = redis_conn
        .get(&id)
        .expect("Failed to check if custom short URL exists");

    if let Some(val) = existing_val {
        if !val.is_empty() {
            return HttpResponse::Forbidden().json(serde_json::json!({
                "error": "URL Custom short is already in use"
            }));
        }
    }

    // Set expiry (default to 24 hours if not provided)
    let expiry_hours = body.expire_at.unwrap_or(24);

    // Store the URL in Redis with expiry
    let expiry_seconds = expiry_hours * 3600;
    let _: () = redis_conn
        .set_ex(&id, &enforced_url, expiry_seconds as u64)
        .expect("Failed to store URL in Redis");

    // Decrement rate limit counter
    let remaining_quota: i32 = redis_conn
        .decr(&key, 1)
        .expect("Failed to decrement rate limit");

    // Calculate rate limit reset time in minutes
    let rate_limit_reset = limit / 60;

    let response = Response {
        url: enforced_url,
        custom_short_url: format!("{}/{}", configuration.application.domain, id),
        expiry: expiry_hours,
        x_rate_remaining: remaining_quota,
        x_rate_limit_reset: rate_limit_reset as i32,
    };

    HttpResponse::Ok().json(response)
}
