use serde::Serialize;

#[derive(Serialize)]
pub struct UrmInfo {
  pub brand: &'static str,
  pub product_name: &'static str,
  pub version: &'static str,
}

impl Default for UrmInfo {
  fn default() -> Self {
    UrmInfo {
      brand: "Unified Repository Manager",
      product_name: "Unified Repository Manager",
      version: "v0.1.0"
    }
  }
}

#[derive(Default, Serialize)]
pub struct Repositories {
  pub number: u64,
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
