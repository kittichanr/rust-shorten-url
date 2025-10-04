use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};
use redis::{Client, Commands};
use secrecy::ExposeSecret;

use crate::{configuration::Settings, routes::shorten::post::post_shorten};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let client = redis::Client::open(configuration.redis_uri.expose_secret()).unwrap();
        let pool = r2d2::Pool::new(client).unwrap();

        let server = run(listener, pool, configuration).await?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn run(
    listener: TcpListener,
    redis_pool: r2d2::Pool<Client>,
    configuration: Settings,
) -> Result<Server, std::io::Error> {
    let redis_pool = web::Data::new(redis_pool);
    let configuration = web::Data::new(configuration);
    let server = HttpServer::new(move || {
        App::new()
            .route("/hello", web::get().to(manual_hello))
            .route("/shorten", web::post().to(post_shorten))
            .app_data(redis_pool.clone())
            .app_data(configuration.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn manual_hello(redis_pool: web::Data<r2d2::Pool<Client>>) -> impl Responder {
    let mut redis_conn = redis_pool
        .get()
        .expect("Failed to get Redis connection from pool");

    let _: () = redis_conn
        .set("my_key", 42)
        .expect("failed to execute SET for 'my_key'");

    let val: i32 = redis_conn
        .get("my_key")
        .expect("failed to execute GET for 'my_key'");

    println!("counter = {}", val);
    HttpResponse::Ok().body("Hey there!")
}
