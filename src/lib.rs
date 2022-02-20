#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

// only c version of trainer works for this
extern crate word2vec;


use rocket::response::Redirect;
use rocket::{Request, Route, State, Rocket, Build};
use word2vec::wordvectors::WordVector;
use rocket::serde::json::{Value, json};

#[get("/")]
async fn index() -> Redirect {
    Redirect::to(uri!(help))
}

#[get("/help")]
fn help(routes: &State<Vec<Route>>) -> Value {
    json!(routes
        .iter()
        .map(|route| {
            format!(
                "{} {}{}",
                route.method.as_str(),
                route.uri.path(),
                match route.uri.query() {
                    Some(params) => format!("?{}", params),
                    None => "".to_string(),
                }
            )
        })
        .collect::<Vec<String>>())
}

#[get("/word_count")]
async fn word_count(model: &State<&WordVector>) -> Value {
    json!(model.word_count().await)
}

#[get("/vector_size")]
async fn vector_size(model: &State<&WordVector>) -> Value {
    json!(model.get_col_count().await)
}

#[get("/vector/<word>")]
async fn vector(model: &State<&WordVector>, word: String) -> Value {
    json!(model.get_vector(&word).await)
}

#[get("/analogy?<pos>&<neg>&<n>")]
async fn analogy(model: &State<&WordVector>, pos: String, neg: String, n: Option<u32>) -> Value {
    json!(model.analogy(
        &pos.split(' ').collect::<Vec<&str>>(),
        &neg.split(' ').collect::<Vec<&str>>(),
        match n {
            Some(n) => n as usize,
            None => 10,
        }
    ).await)
}

#[get("/cosine/<word>?<n>")]
async fn cosine(model: &State<&WordVector>, word: String, n: Option<u32>) -> Value {
    json!(model.cosine(
        &word,
        match n {
            Some(n) => n as usize,
            None => 10,
        }
    ).await)
}

#[get("/show/me/to/<target>/what/<comparison>/is/to/<origin>")]
async fn analogynice(
    model: &State<&WordVector>,
    target: String,
    comparison: String,
    origin: String,
) -> Value {
    json!(model.analogy(&[&target, &comparison], &[&origin], 1).await)
}

#[catch(404)]
async fn not_found(req: &'_ Request<'_>) -> Value {
    json!(format!("Error 404, '{}' not found. See /help.", req.uri()))
}

pub async fn build_rocket(filename: &str) -> Rocket<Build> {
    let static_model: &'static WordVector = Box::leak(Box::new(
        WordVector::load_from_binary(filename).await.expect("Unable to load word vector model"),
    ));
    let routes = routes![
        index,
        word_count,
        vector_size,
        vector,
        cosine,
        analogy,
        analogynice,
        help,
    ];
    rocket::build()
        .mount("/", routes.clone())
        .manage(static_model)
        .manage(routes)
        .register("/", catchers![not_found])
}