use actix_web::{HttpRequest, HttpResponse, Responder, web};
use redis::{Client, Commands};

pub async fn get_resolve(
    redis_pool: web::Data<r2d2::Pool<Client>>,
    req: HttpRequest,
) -> impl Responder {
    let url_param = req.match_info().get("url").unwrap_or("");

    if url_param.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "URL parameter is required"
        }));
    }

    let mut redis_conn = redis_pool
        .get()
        .expect("Failed to get Redis connection from pool");

    let value: Result<Option<String>, redis::RedisError> = redis_conn.get(url_param);

    match value {
        Ok(Some(original_url)) => {
            let _: () = redis_conn
                .incr("counter", 1)
                .expect("Failed to increment counter");

            HttpResponse::MovedPermanently()
                .append_header(("Location", original_url))
                .finish()
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "short-url not found in db"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}
