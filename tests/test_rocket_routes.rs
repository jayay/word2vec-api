#[cfg(test)]
mod test {
    extern crate rocket;
    extern crate word2vec_api_lib;
    extern crate serde_json;
    extern crate serde;

    use rocket::local::asynchronous::{Client, LocalResponse};
    use rocket::http::{Status, ContentType};
    use word2vec_api_lib::*;
    use serde::Deserialize;

    #[tokio::test]
    async fn test_index() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/");
        let response = request.dispatch().await;
        assert_eq!(Status::SeeOther, response.status());
        assert_eq!(None, response.content_type());
        assert!(response.into_string().await.is_none());
    }


    #[tokio::test]
    async fn test_help() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/help");
        let response = request.dispatch().await;
        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        assert_eq!(Some(r###"["GET /","GET /word_count","GET /vector_size","GET /vector/<word>","GET /cosine/<word>?<n>","GET /analogy?<pos>&<neg>&<n>","GET /show/me/to/<target>/what/<comparison>/is/to/<origin>","GET /help"]"###.to_string()), response.into_string().await);
    }

    #[tokio::test]
    async fn test_word_count() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/word_count");
        let response = request.dispatch().await;
        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        assert_eq!(Some("86588".to_string()), response.into_string().await);
    }


    #[tokio::test]
    async fn test_vector_size() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/vector_size");
        let response = request.dispatch().await;
        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        assert_eq!(Some("100".to_string()), response.into_string().await);
    }


     #[tokio::test]
    async fn test_404() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/a");
        let response = request.dispatch().await;
        assert_eq!(Status::NotFound, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        assert_eq!(Some(r###""Error 404, '/a' not found. See /help.""###.to_string()), response.into_string().await);
    }

    #[tokio::test]
    async fn test_vector() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/vector/rust");
        let response = request.dispatch().await;
        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        let response_vec = serde_json::from_str::<Vec<f32>>(&response.into_string().await.unwrap()).unwrap();
        assert_eq!(100, response_vec.len());
        let average = response_vec.iter().sum::<f32>() as f32 / response_vec.len() as f32;
        assert_ne!(average, 0.0);
    }


    #[derive(Deserialize, Debug, PartialEq)]
    struct Word2VecResult(String, f32);

    #[tokio::test]
    async fn test_cosine_default() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/cosine/computer");
        let response = request.dispatch().await;
        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        let response_vec = serde_json::from_str::<Vec<Word2VecResult>>(&response.into_string().await.unwrap()).unwrap();
        assert_eq!(10, response_vec.len());
    }

    #[tokio::test]
    async fn test_cosine_n() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/cosine/software?n=3");
        let response = request.dispatch().await;
        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        let response_vec = serde_json::from_str::<Vec<Word2VecResult>>(&response.into_string().await.unwrap()).unwrap();
        assert_eq!(3, response_vec.len());

        let expected_result = vec![
            Word2VecResult("hardware".to_string(), 0.7883262634277344),
            Word2VecResult("compiler".to_string(), 0.7580281496047974),
            Word2VecResult("multitasking".to_string(), 0.7403385639190674),
        ];
        assert_eq!(expected_result, response_vec);
    }

    #[tokio::test]
    async fn test_analogy_n() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/analogy/?pos=woman+king&neg=man&n=1");
        let response = request.dispatch().await;
        let _ = check_analogy_1(response).await;
    }


    async fn check_analogy_1(response: LocalResponse<'_>) {
        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        let result = response.into_string().await.unwrap();
        let response_vec: Vec<(String, f32)> =
            serde_json::from_slice((&result).as_ref()).unwrap();
        assert_eq!(1, response_vec.len());
        assert_eq!("queen".to_string(), response_vec[0].0);
        assert!((0.24765503406524658 - &response_vec[0].1).abs() < 0.00001);
    }

    #[tokio::test]
    async fn test_analogy_default() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/analogy/?pos=human+meat&neg=dog");
        let response = request.dispatch().await;
        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        let response_vec = serde_json::from_str::<Vec<Word2VecResult>>(&response.into_string().await.unwrap()).unwrap();
        assert_eq!(10, response_vec.len());

        let expected_result = vec![
            Word2VecResult("food".to_string(), 0.28690820932388306),
            Word2VecResult("agricultural".to_string(), 0.28199446201324463),
            Word2VecResult("cultivation".to_string(), 0.27436479926109314),
            Word2VecResult("nutrition".to_string(), 0.27405092120170593),
            Word2VecResult("agriculture".to_string(), 0.2718198299407959),
            Word2VecResult("nutritional".to_string(), 0.26885557174682617),
            Word2VecResult("subsistence".to_string(), 0.2645898461341858),
            Word2VecResult("dairy".to_string(), 0.2601871192455292),
            Word2VecResult("aspartame".to_string(), 0.2572169601917267),
            Word2VecResult("dietary".to_string(), 0.2545112371444702),
        ];

        for (index, vec_result) in expected_result.iter().enumerate() {
            assert_eq!(vec_result.0, response_vec[index].0);
            assert!((vec_result.1 - response_vec[index].1).abs() < 0.0000001);
        }
    }


     #[tokio::test]
    async fn test_analogy_nice_default() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/show/me/to/woman/what/king/is/to/man/");
        let response = request.dispatch().await;
        let _ = check_analogy_1(response).await;
    }


    #[tokio::test]
    #[should_panic]
    async fn test_build_rocket_invalid_filename() {
        let _ = build_rocket("unknown").await.ignite().await;
    }


    #[tokio::test]
    #[should_panic]
    async fn test_build_rocket_invalid_filecontents() {
        let _ = build_rocket("Cargo.toml").await.ignite().await;
    }


    async fn get_rocket_client() -> Client {
        let filename = "tests/model/trained-small.bin";
        let rocket = build_rocket(filename).await;
        let client = Client::untracked(rocket).await.expect("valid rocket instance");
        client
    }
}
