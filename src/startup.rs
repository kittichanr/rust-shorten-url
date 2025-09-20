use std::net::TcpListener;

use actix_session::storage::RedisSessionStore;
use actix_web::{App, HttpResponse, HttpServer, Responder, cookie::Key, dev::Server, web};
use secrecy::{ExposeSecret, SecretString};

use crate::configuration::Settings;

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
        let server = run(listener, configuration.redis_uri).await?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn run(listener: TcpListener, redis_uri: SecretString) -> Result<Server, std::io::Error> {
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret())
        .await
        .expect("Failed to create Redis session store");
    let server = HttpServer::new(move || App::new().route("/hello", web::get().to(manual_hello)))
        .listen(listener)?
        .run();

    Ok(server)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
