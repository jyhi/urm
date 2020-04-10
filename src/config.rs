use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Collection {
  pub products: String,
  pub repositories: String,
}

impl Default for Collection {
  fn default() -> Self {
    Collection {
      products: "products".to_string(),
      repositories: "repositories".to_string(),
    }
  }
}

#[derive(Serialize)]
pub struct UrmConfig {
  pub brand: String,
  pub product_name: String,
  pub version: String,
  pub mount_point: String,
  pub collection: Collection,
}

impl From<UrmConfigFile> for UrmConfig {
  fn from(file: UrmConfigFile) -> Self {
    UrmConfig {
      brand: file.urm.brand.unwrap_or("Unified Repository Manager".to_string()),
      product_name: file.urm.product_name.unwrap_or(env!("CARGO_PKG_NAME").to_string()),
      version: file.urm.version.unwrap_or(env!("CARGO_PKG_VERSION").to_string()),
      mount_point: file.urm.mount_point.unwrap_or("/".to_string()),
      collection: if let Some(coll) = file.urm.collection {
        Collection {
          products: coll.products.unwrap_or("products".to_string()),
          repositories: coll.repositories.unwrap_or("repositories".to_string()),
        }
      } else {
        Default::default()
      }
    }
  }
}

#[derive(Default, Deserialize)]
struct CollectionFile {
  products: Option<String>,
  repositories: Option<String>,
}

#[derive(Default, Deserialize)]
struct UrmFile {
  brand: Option<String>,
  product_name: Option<String>,
  version: Option<String>,
  mount_point: Option<String>,
  collection: Option<CollectionFile>,
}

#[derive(Default, Deserialize)]
struct UrmConfigFile {
  urm: UrmFile
}

impl UrmConfigFile {
  fn from_file(filename: String) -> Self {
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

pub fn read_config_file(filename: String) -> UrmConfig {
  UrmConfigFile::from_file(filename).into()
}
