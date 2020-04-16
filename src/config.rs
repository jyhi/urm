use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Static {
  pub bootstrap_css: String,
  pub bootstrap_bundle_js: String,
  pub jquery_js: String,
}

impl Default for Static {
  fn default() -> Self {
    Static {
      bootstrap_css: "/css/bootstrap.min.css".to_string(),
      bootstrap_bundle_js: "/js/bootstrap.bundle.min.js".to_string(),
      jquery_js: "/js/jquery.min.js".to_string(),
    }
  }
}

#[derive(Serialize)]
pub struct Collection {
  pub products: String,
  pub repositories: String,
  pub users: String,
}

impl Default for Collection {
  fn default() -> Self {
    Collection {
      products: "products".to_string(),
      repositories: "repositories".to_string(),
      users: "users".to_string(),
    }
  }
}

#[derive(Serialize)]
pub struct Pbkdf2 {
  pub iterations: u32,
}

impl Default for Pbkdf2 {
  fn default() -> Self {
    Pbkdf2 {
      iterations: 10000,
    }
  }
}

#[derive(Serialize)]
pub struct UrmConfig {
  pub brand: String,
  pub product_name: String,
  pub version: String,
  pub mount_point: String,
  pub r#static: Static,
  pub collection: Collection,
}

impl From<UrmConfigFile> for UrmConfig {
  fn from(file: UrmConfigFile) -> Self {
    UrmConfig {
      brand: file.urm.brand.unwrap_or("Unified Repository Manager".to_string()),
      product_name: file.urm.product_name.unwrap_or(env!("CARGO_PKG_NAME").to_string()),
      version: file.urm.version.unwrap_or(env!("CARGO_PKG_VERSION").to_string()),
      mount_point: file.urm.mount_point.unwrap_or("/".to_string()),
      r#static: if let Some(s) = file.urm.r#static {
        Static {
          bootstrap_css: s.bootstrap_css.unwrap_or("/css/bootstrap.min.css".to_string()),
          bootstrap_bundle_js: s.bootstrap_bundle_js.unwrap_or("/js/bootstrap.bundle.min.js".to_string()),
          jquery_js: s.jquery_js.unwrap_or("/js/jquery.min.js".to_string()),
        }
      } else {
        Static::default()
      },
      collection: if let Some(coll) = file.urm.collection {
        Collection {
          products: coll.products.unwrap_or("products".to_string()),
          repositories: coll.repositories.unwrap_or("repositories".to_string()),
          users: coll.users.unwrap_or("users".to_string())
        }
      } else {
        Default::default()
      },
    }
  }
}

#[derive(Default, Deserialize)]
struct StaticFile {
  bootstrap_css: Option<String>,
  bootstrap_bundle_js: Option<String>,
  jquery_js: Option<String>,
}

#[derive(Default, Deserialize)]
struct CollectionFile {
  products: Option<String>,
  repositories: Option<String>,
  users: Option<String>,
}

#[derive(Default, Deserialize)]
struct UrmFile {
  brand: Option<String>,
  product_name: Option<String>,
  version: Option<String>,
  mount_point: Option<String>,
  r#static: Option<StaticFile>,
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
