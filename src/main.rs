use actix_web::{
    web::{post, resource, scope},
    App, HttpServer,
};

mod handlers;
use handlers::{display, solve};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "localhost:7878";
    println!("Listening for requests at http://{}", addr);

    HttpServer::new(|| {
        App::new().service(
            scope("/api")
                .service(resource("/solve").route(post().to(solve)))
                .service(resource("/display").route(post().to(display))),
        )
    })
    .bind(addr)
    .unwrap()
    .run()
    .await
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;
    use actix_web::{http, test};
    use bytes::Bytes;

    #[test]
    fn solve_ok() {
        async {
            let mut app = test::init_service(
                App::new().service(
                    scope("/api")
                        .service(resource("/solve").route(post().to(solve)))
                        .service(resource("/display").route(post().to(display))),
                ),
            )
            .await;

            let req = test::TestRequest::with_header("content-type", "application/json")
                    .method(http::Method::POST)
                    .uri("/api/solve")
                    .set_payload(r#"{"puzzle": "700000600060001070804020005000470000089000340000039000600050709010300020003000004"}"#)
                    .to_request();

            let resp = test::call_service(&mut app, req).await;
            assert!(resp.status().is_success());

            let body = test::read_body(resp).await;
            assert_eq!(body, Bytes::from_static(
                r#"{"status":"success","data":"791543682562981473834726915356478291289615347147239568628154739415397826973862154","message":""}"#
                    .as_bytes())
            );
        };
    }

    #[test]
    fn display_ok() {
        async {
            let mut app = test::init_service(
                App::new().service(
                    scope("/api")
                        .service(resource("/solve").route(post().to(solve)))
                        .service(resource("/display").route(post().to(display))),
                ),
            )
            .await;

            let req = test::TestRequest::with_header("content-type", "application/json")
                    .method(http::Method::POST)
                    .uri("/api/display")
                    .set_payload(r#"{"puzzle": "309800000000500000250009600480000097700000005930000061008300056000006000000007403"}"#)
                    .to_request();

            let resp = test::call_service(&mut app, req).await;
            assert!(resp.status().is_success());

            let body = test::read_body(resp).await;
            assert_eq!(body, Bytes::from_static(
                concat!(r#"{"status":"success","data":["3 0 9 |8 0 0 |0 0 0 ","0 0 0 |5 0 0 |0 0 0 ","2 5 0 |0 0 9 |6 0 0 ","------+------+------","#,
                    r#"4 8 0 |0 0 0 |0 9 7 ","7 0 0 |0 0 0 |0 0 5 ","9 3 0 |0 0 0 |0 6 1 ","------+------+------","#,
                    r#"0 0 8 |3 0 0 |0 5 6 ","0 0 0 |0 0 6 |0 0 0 ","0 0 0 |0 0 7 |4 0 3 "],"message":""}"#
                )
                .as_bytes())
            );
        };
    }

    #[test]
    fn solve_err_puzzle() {
        async {
            let mut app = test::init_service(
                App::new().service(
                    scope("/api")
                        .service(resource("/solve").route(post().to(solve)))
                        .service(resource("/display").route(post().to(display))),
                ),
            )
            .await;

            let req = test::TestRequest::with_header("content-type", "application/json")
            .method(http::Method::POST)
            .uri("/api/solve")
            .set_payload(r#"{"puzzle": "X00000600060001070804020005000470000089000340000039000600050709010300020003000004"}"#)
            .to_request();

            let resp = test::call_service(&mut app, req).await;
            assert!(resp.status().is_success());

            let body = test::read_body(resp).await;
            assert_eq!(body, Bytes::from_static(
                r#"{"status":"fail","data":"","message":"Invalid Grid.  Provide a string of 81 digits with 0 or . for empties."}"#
                .as_bytes())
            );
        };
    }

    #[test]
    fn solve_err_json() {
        async {
            let mut app = test::init_service(
                App::new().service(
                    scope("/api")
                        .service(resource("/solve").route(post().to(solve)))
                        .service(resource("/display").route(post().to(display))),
                ),
            )
            .await;

            let req = test::TestRequest::with_header("content-type", "application/json")
                .method(http::Method::POST)
                .uri("/api/solve")
                .set_payload(r#"{"xuzzle": "700000600060001070804020005000470000089000340000039000600050709010300020003000004"}"#)
                .to_request();

            let resp = test::call_service(&mut app, req).await;
            assert!(resp.status().is_success());

            let body = test::read_body(resp).await;
            assert_eq!(body, Bytes::from_static(
                r#"{"status":"fail","data":"","message":"Invalid Grid.  Provide a string of 81 digits with 0 or . for empties."}"#
                .as_bytes())
            );
        };
    }
}
