use secrecy::SecretString;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub redis_uri: SecretString,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}
