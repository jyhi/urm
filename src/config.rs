use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Collection {
  #[serde(default = "Collection::default_products")]
  pub products: String,
  #[serde(default = "Collection::default_repositories")]
  pub repositories: String,
  #[serde(default = "Collection::default_users")]
  pub users: String,
}

impl Collection {
  fn default_products() -> String {
    String::from("products")
  }

  fn default_repositories() -> String {
    String::from("repositories")
  }

  fn default_users() -> String {
    String::from("users")
  }
}

impl Default for Collection {
  fn default() -> Self {
    Collection {
      products: Collection::default_products(),
      repositories: Collection::default_repositories(),
      users: Collection::default_users(),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct UrmConfig {
  #[serde(default = "UrmConfig::default_brand")]
  pub brand: String,
  #[serde(default = "UrmConfig::default_product_name")]
  pub product_name: String,
  #[serde(default = "UrmConfig::default_version")]
  pub version: String,
  #[serde(default = "UrmConfig::default_mount_point")]
  pub mount_point: String,
  #[serde(default)]
  pub collection: Collection,
}

impl UrmConfig {
  fn default_brand() -> String {
    String::from("Unified Repository Manager")
  }

  fn default_product_name() -> String {
    String::from(env!("CARGO_PKG_NAME"))
  }

  fn default_version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
  }

  fn default_mount_point() -> String {
    String::from("/")
  }

  pub fn from_file(filename: &str) -> Self {
    match std::fs::read_to_string(&filename) {
      Ok(s) => {
        match toml::from_str(&s) {
          Ok(c) => c,
          Err(e) => {
            eprintln!("Failed to parse the configuration file: {}. Using the default.", e);
            Self::default()
          }
        }
      }
      Err(_) => {
        Self::default()
      }
    }
  }
}

impl Default for UrmConfig {
  fn default() -> Self {
    UrmConfig {
      brand: UrmConfig::default_brand(),
      product_name: UrmConfig::default_product_name(),
      version: UrmConfig::default_version(),
      mount_point: UrmConfig::default_mount_point(),
      collection: Collection::default(),
    }
  }
}
