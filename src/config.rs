use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Urm {
  pub brand: Option<String>
}

#[derive(Default, Deserialize)]
pub struct UrmConfig {
  pub urm: Option<Urm>
}

impl UrmConfig {
  pub fn from_file(filename: String) -> Self {
    match std::fs::read_to_string(&filename) {
      Ok(s) => {
        match toml::from_str(&s) {
          Ok(c) => c,
          Err(e) => {
            eprintln!("Failed to parse the configuration file: {}. Using the default.", e);
            Default::default()
          }
        }
      }
      Err(_) => {
        Default::default()
      }
    }
  }
}
