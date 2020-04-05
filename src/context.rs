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
pub struct Repository {
  pub name: String,
  pub load: u64,
}

#[derive(Default, Serialize)]
pub struct Repositories {
  pub number: u64,
  pub list: Vec<Repository>,
}

#[derive(Default, Serialize)]
pub struct Products {
  pub number: u64,
}

#[derive(Serialize)]
pub struct UrmContext<'a> {
  pub urm: &'a UrmInfo,
  pub repositories: &'a Repositories,
  pub products: &'a Products,
}
