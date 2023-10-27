// use the fs module to read proxy.toml
use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    name: String,
    version: String,
    pub base_url: String,
}

impl Service {
    pub fn new(name: &str, base_url: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            base_url: base_url.to_string(),
            version: version.to_string(),
        }
    }
}
pub fn parse_config(service_id: &str) -> Result<Service, String> {
    // read the service configuration, or throw error
    // let Some(config) = fs::read_to_string("./proxy.toml").ok() else {
    //     return Err(String::from("error reading config"));
    // };
    let config = std::include_bytes!("../proxy.toml");
    let config = String::from_utf8_lossy(config);
    //  convert the file into Table destructure provided by Toml parser
    let config = config.parse::<Table>().unwrap();
    let Some(service) = &config["services"].get(service_id) else {
        return Err(String::from("error parsing config"));
    };

    // convert to Services Struct
    let name = service_id;
    let version = format!("v{}", service.get("version").unwrap().as_str().unwrap());
    let base_url = service.get("base_url").unwrap().as_str().unwrap();

    Ok(Service::new(name, base_url, &version))
}
