use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use serde::Serialize;
use crate::repository::Repository;
use crate::context::{UrmInfo, Tag, Attribute};

#[derive(Serialize)]
pub struct Product {
  pub pn: String,
  pub name: String,
  pub amount: u64,
  pub r#in: Repository,
  pub on: String,
  pub tags: Vec<Tag>,
  pub attributes: Vec<Attribute>,
}

#[derive(Serialize)]
pub struct ProductContext<'a> {
  pub urm: &'a UrmInfo,
  pub product: Product,
}

impl<'a> ProductContext<'a> {
  fn test(urm_info: &'a UrmInfo, pn: String) -> Self {
    let product = Product {
      pn: pn.clone(),
      name: "Epic Bacon".to_string(),
      amount: 42,
      r#in: Repository { ln_p: "Z12345678".to_string(), name: "Test Repository".to_string(), load: 42, tags: vec![], has: None },
      on: "Y12345678".to_string(),
      tags: vec![
        Tag { name: "test product".to_string() },
      ],
      attributes: vec![
        Attribute { key: "test attr key".to_string(), value: "Test attribute value".to_string() },
      ],
    };

    ProductContext {
      urm: &urm_info,
      product: product,
    }
  }
}

#[get("/product/<pn>", format = "json")]
pub fn api(urm_info: State<UrmInfo>, pn: String) -> JsonValue {
  let ctx = ProductContext::test(&urm_info, pn.clone());
  json!(ctx)
}

#[get("/product/<pn>", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, pn: String) -> Template {
  let ctx = ProductContext::test(&urm_info, pn.clone());
  Template::render("product", ctx)
}
