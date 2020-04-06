use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use serde::Serialize;
use crate::product::Product;
use crate::repository::Repository;
use crate::context::{UrmInfo, PageInfo, Tag, Attribute};

#[derive(Serialize)]
pub struct ProductsContext<'a> {
  pub urm: &'a UrmInfo,
  pub page: &'a PageInfo,
  pub products: Vec<Product>,
}

impl<'a> ProductsContext<'a> {
  fn test(urm_info: &'a UrmInfo, page_info: &'a PageInfo) -> Self {
    let products = vec![
      Product {
        pn: "012345".to_string(),
        name: "Epic Bacon".to_string(),
        amount: 42,
        r#in: Repository { ln_p: "Z12345678".to_string(), name: "Test Repository".to_string(), load: 42, tags: vec![], has: None },
        on: "Y12345678".to_string(),
        tags: vec![
          Tag { name: "test product".to_string() },
        ],
        attributes: vec![
          Attribute { key: "testattrkey".to_string(), value: "Test attribute value".to_string() },
        ],
      },
    ];

    ProductsContext {
      urm: &urm_info,
      page: &page_info,
      products: products,
    }
  }
}

#[get("/products", format = "json")]
pub fn api() -> JsonValue {
  json!({
    "error": true,
    "desc": "Not implemented"
  })
}

#[get("/products", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>) -> Template {
  let page_info = PageInfo { current: 1, min: 1, max: 1 };

  let ctx = ProductsContext::test(&urm_info, &page_info);
  Template::render("products", ctx)
}
