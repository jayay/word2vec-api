#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
// only c version of trainer works for this
extern crate word2vec;

use rocket::fairing::AdHoc;
use rocket::response::Redirect;
use rocket::{Request, Route, State, Rocket};
use rocket_contrib::json::JsonValue;
use word2vec::wordvectors::WordVector;


#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(help))
}

#[get("/help")]
fn help(routes: State<Vec<Route>>) -> JsonValue {
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
fn word_count(model: State<&WordVector>) -> JsonValue {
    json!(model.word_count())
}

#[get("/vector_size")]
fn vector_size(model: State<&WordVector>) -> JsonValue {
    json!(model.get_col_count())
}

#[get("/vector/<word>")]
fn vector(model: State<&WordVector>, word: String) -> JsonValue {
    json!(model.get_vector(&word))
}

#[get("/analogy?<pos>&<neg>&<n>")]
fn analogy(model: State<&WordVector>, pos: String, neg: String, n: Option<u32>) -> JsonValue {
    json!(model.analogy(
        pos.split(' ').collect::<Vec<&str>>(),
        neg.split(' ').collect::<Vec<&str>>(),
        match n {
            Some(n) => n as usize,
            None => 10,
        }
    ))
}

#[get("/cosine/<word>?<n>")]
fn cosine(model: State<&WordVector>, word: String, n: Option<u32>) -> JsonValue {
    json!(model.cosine(
        &word,
        match n {
            Some(n) => n as usize,
            None => 10,
        }
    ))
}

#[get("/show/me/to/<target>/what/<comparison>/is/to/<origin>")]
fn analogynice(
    model: State<&WordVector>,
    target: String,
    comparison: String,
    origin: String,
) -> JsonValue {
    json!(model.analogy(vec![&target, &comparison], vec![&origin], 1))
}

#[catch(404)]
fn not_found(req: &Request) -> JsonValue {
    json!(format!("Error 404, '{}' not found. See /help.", req.uri()))
}

pub fn build_rocket(filename: &str) -> Rocket {
    let static_model: &'static WordVector = Box::leak(Box::new(
        WordVector::load_from_binary(filename).expect("Unable to load word vector model"),
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
    rocket::ignite()
        .mount("/", routes.clone())
        .manage(static_model)
        .manage(routes)
        .attach(AdHoc::on_response("Dummy", |_request, response| {
            response.remove_header("Server");
        }))
        .register(catchers![not_found])
}