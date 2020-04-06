use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use crate::context::*;

#[get("/product/<pn>", format = "json")]
pub fn api(pn: String) -> JsonValue {
  json!({
    "error": true,
    "desc": "Not implemented",
    "P/N": pn,
  })
}

#[get("/product/<pn>", format = "html", rank = 1)]
pub fn ui(pn: String, urm_info: State<UrmInfo>) -> Template {
  // TODO: Fetch from DB
  let repo_list = vec![
    Repository { ln_p: "T0803080".to_string(), name: "T8-308".to_string(), load: 114000, tags: vec![Tag { name: "testing".to_string() }] },
  ];
  let product = Product { pn: pn, name: "Epic Bacon".to_string(), r#in: repo_list.clone(), on: "H0010300".to_string(), amount: 42 };
  let page = Page { current: 1, min: 1, max: 1 };

  let ctx = UrmProductContext {
    urm: &urm_info,
    repositories: &repo_list,
    product: &product,
    page: &page,
  };

  Template::render("product", ctx)
}
