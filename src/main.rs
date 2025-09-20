use rust_shorten_url::configuration;
use secrecy::ExposeSecret;

fn main() {
    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    println!("{:?}", configuration.application);
    println!("{:?}", configuration.redis_uri.expose_secret());
}
