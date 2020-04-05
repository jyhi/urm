use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use crate::context::*;

#[get("/repository/<ln_p>", format = "json")]
pub fn api(ln_p: String) -> JsonValue {
  json!({
    "error": true,
    "desc": "Not implemented",
    "L/N-P": ln_p,
  })
}

#[get("/repository/<ln_p>", format = "html", rank = 1)]
pub fn ui(ln_p: String, urm_info: State<UrmInfo>) -> Template {
  // TODO: Fetch from DB
  let product_list = vec![
    Product { pn: "100040".to_string(), name: "Foo Bar".to_string(), r#in: "T8-308".to_string(), amount: 147 },
    Product { pn: "100041".to_string(), name: "Lorem ipsum".to_string(), r#in: "T8-308".to_string(), amount: 255 },
    Product { pn: "100042".to_string(), name: "Epic Bacon".to_string(), r#in: "T8-308".to_string(), amount: 42 },
  ];
  let repository = Repository { ln_p: ln_p, name: "T8-308".to_string(), load: 444, tags: vec![Tag { name: "testing".to_string() }] };
  let products = Products { number: product_list.len() as u64, list: product_list };
  let page = Page { current: 1, min: 1, max: 1 };

  let ctx = UrmRepositoryContext {
    urm: &urm_info,
    repository: &repository,
    products: &products,
    page: &page,
  };

  Template::render("repository", ctx)
}
