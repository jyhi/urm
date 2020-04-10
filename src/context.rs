use serde::Serialize;

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
