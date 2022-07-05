#[cfg(test)]
mod test {
    extern crate rocket;
    extern crate word2vec_api_lib;
    extern crate serde_json;
    extern crate serde;

    use rocket::local::asynchronous::{Client};
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
        assert!(response.body().is_none());
    }

    #[tokio::test]
    async fn test_help() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/help");
        let response = request.dispatch().await;

        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());

        assert_eq!(r###"["GET /","GET /word_count","GET /vector_size","GET /vector/<word>","GET /cosine/<word>?<n>","GET /analogy?<pos>&<neg>&<n>","GET /show/me/to/<target>/what/<comparison>/is/to/<origin>","GET /help"]"###.to_string(), response.into_string().await.unwrap());
    }

    #[tokio::test]
    async fn test_word_count() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/word_count");
        let response = request.dispatch().await;
        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        assert_eq!("86588".to_string(), response.into_string().await.unwrap());
    }


    #[tokio::test]
    async fn test_vector_size() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/vector_size");
        let response = request.dispatch().await;
        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        assert_eq!("100".to_string(), response.into_string().await.unwrap());
    }


     #[tokio::test]
    async fn test_404() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/a");
        let response = request.dispatch().await;
        assert_eq!(Status::NotFound, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());
        assert_eq!(r###""Error 404, '/a' not found. See /help.""###.to_string(), response.into_string().await.unwrap());
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
            Word2VecResult("hardware".to_string(), 0.788_326_26),
            Word2VecResult("compiler".to_string(), 0.758_028_15),
            Word2VecResult("multitasking".to_string(), 0.740_338_56),
        ];
        assert_eq!(expected_result, response_vec);
    }

    #[tokio::test]
    async fn test_analogy_n() {
        let rocket = get_rocket_client().await;
        let request = rocket.get("/analogy/?pos=woman+king&neg=man&n=1");
        let response = request.dispatch().await;

        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());

        let body = &response.into_string().await.unwrap();
        let response_vec: Vec<Word2VecResult> = serde_json::from_str(body).unwrap();
        assert_eq!(1, response_vec.len());
        assert_eq!("queen".to_string(), response_vec[0].0);
        assert!((0.247_655_03 - response_vec[0].1).abs() < 0.00001);
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
            Word2VecResult("food".to_string(), 0.286_908_2),
            Word2VecResult("agricultural".to_string(), 0.281_994_46),
            Word2VecResult("cultivation".to_string(), 0.274_364_8),
            Word2VecResult("nutrition".to_string(), 0.274_050_92),
            Word2VecResult("agriculture".to_string(), 0.271_819_83),
            Word2VecResult("nutritional".to_string(), 0.268_855_57),
            Word2VecResult("subsistence".to_string(), 0.264_589_85),
            Word2VecResult("dairy".to_string(), 0.260_187_12),
            Word2VecResult("aspartame".to_string(), 0.257_216_96),
            Word2VecResult("dietary".to_string(), 0.254_511_24),
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
        assert_eq!(Status::Ok, response.status());
        assert_eq!(Some(ContentType::JSON), response.content_type());

        let body = response.into_string().await.unwrap();
        // let mut clone = &mut body.to_bytes().await.unwrap();
        let response_vec: Vec<Word2VecResult> = serde_json::from_str(&body).unwrap();
        assert_eq!(1, response_vec.len());
        assert_eq!("queen".to_string(), response_vec[0].0);
        assert!((0.247_655_03 - response_vec[0].1).abs() < 0.00001);
    }


    #[tokio::test]
    #[should_panic]
    async fn test_build_rocket_invalid_filename() {
        let _ = build_rocket("unknown");
    }


    #[tokio::test]
    #[should_panic]
    async fn test_build_rocket_invalid_filecontents() {
        let _ = build_rocket("Cargo.toml");
    }


    async fn get_rocket_client() -> Client {
        let filename = "tests/model/trained-small.bin";
        let rocket = build_rocket(filename);
        Client::untracked(rocket).await.expect("valid rocket instance")
    }
}
