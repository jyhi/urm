use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Urm {
  pub brand: Option<String>
}

#[derive(Default, Deserialize)]
pub struct UrmConfig {
  pub urm: Option<Urm>
}
