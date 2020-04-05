use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use crate::context::*;

#[get("/repositories", format = "json")]
pub fn api() -> JsonValue {
  json!({
    "error": true,
    "desc": "Not implemented"
  })
}

#[get("/repositories", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>) -> Template {
  // TODO: Fetch from DB
  let repo_list = vec![
    Repository { name: "Repository 1".to_string(), load: 114000 },
    Repository { name: "Repository 2".to_string(), load: 514 },
  ];
  let product_list = vec![
    Product { pn: "100040".to_string(), name: "Foo Bar".to_string(), r#in: "T8-308".to_string(), amount: 147 },
    Product { pn: "100041".to_string(), name: "Lorem ipsum".to_string(), r#in: "T8-308".to_string(), amount: 255 },
    Product { pn: "100042".to_string(), name: "Epic Bacon".to_string(), r#in: "T8-308".to_string(), amount: 42 },
  ];
  let repositories = Repositories { number: repo_list.len() as u64, list: repo_list };
  let products = Products { number: product_list.len() as u64, list: product_list };
  let page = Page { current: 1, min: 1, max: 3 };

  let ctx = UrmContext {
    urm: &urm_info,
    repositories: &repositories,
    products: &products,
    page: &page,
  };

  Template::render("repositories", ctx)
}
