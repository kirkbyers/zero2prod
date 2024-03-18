use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub local_file_path: String,
}

pub fn get_configuration(config_file_name: Option<&str>) -> Result<Settings, config::ConfigError> {
    let file_name = config_file_name.unwrap_or("configuration.yaml");
    let settings = config::Config::builder()
        .add_source(config::File::new(file_name, config::FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Settings>()
}
