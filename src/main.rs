#[macro_use]
extern crate rocket;

// only c version of trainer works for this
extern crate word2vec;

extern crate word2vec_api_lib;
use word2vec_api_lib::build_rocket;
use std::env;
use rocket::{Build, Rocket};

#[launch]
async fn launch() -> Rocket<Build> {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("No file name given.");
    build_rocket(filename).await
}