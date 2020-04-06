use serde::Serialize;

#[derive(Serialize)]
pub struct UrmInfo {
  pub brand: String,
  pub product_name: &'static str,
  pub version: &'static str,
}

impl Default for UrmInfo {
  fn default() -> Self {
    UrmInfo {
      brand: env!("CARGO_PKG_NAME").to_string(),
      product_name: env!("CARGO_PKG_NAME"),
      version: env!("CARGO_PKG_VERSION")
    }
  }
}

#[derive(Serialize)]
pub struct Tag {
  pub name: String,
}

#[derive(Serialize)]
pub struct Attribute {
  pub key: String,
  pub value: String,
}

#[derive(Serialize)]
pub struct PageInfo {
  pub current: u64,
  pub min: u64,
  pub max: u64,
}
