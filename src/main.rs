use rust_shorten_url::{configuration, startup::Application};

#[tokio::main]
async fn main() {
    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");

    application
        .run_until_stopped()
        .await
        .expect("Failed to run application");
}
