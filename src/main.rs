use actix_web::{server, App, http};

mod handlers;
use handlers::{display, solve};

mod sudoku;

fn main() {
    let addr = "localhost:7878";
    println!("Listening for requests at http://{}", addr);

    server::new(|| App::new()
                    .prefix("/api")
                    .resource("/solve", |r| r.method(http::Method::POST).f(solve))
                    .resource("/display", |r| r.method(http::Method::POST).f(display))
                )
                .bind(addr)
                .unwrap()
                .run();
}

#[cfg(test)]
mod tests {
    use actix_web::{test::TestRequest, Body};
    use super::{http, solve, display};

    #[test]
    fn solve_ok() {
        let resp = TestRequest::with_header("content-type", "application/json")
                .method(http::Method::POST)
                .set_payload(r#"{"puzzle": "700000600060001070804020005000470000089000340000039000600050709010300020003000004"}"#)
                .run(&solve)
                .unwrap();

        assert!(resp.status().is_success());

        assert_eq!(resp.body(), &Body::from_slice(
            r#"{"status":"success","data":"791543682562981473834726915356478291289615347147239568628154739415397826973862154","message":""}"#
                .as_bytes())
        );
    }

 
    #[test]
    fn display_ok() {
        let resp = TestRequest::with_header("content-type", "application/json")
                .method(http::Method::POST)
                .set_payload(r#"{"puzzle": "309800000000500000250009600480000097700000005930000061008300056000006000000007403"}"#)
                .run(&display)
                .unwrap();

        assert!(resp.status().is_success());

        assert_eq!(resp.body(), &Body::from_slice(
            r#"{"status":"success","data":["3 0 9 |8 0 0 |0 0 0 ","0 0 0 |5 0 0 |0 0 0 ","2 5 0 |0 0 9 |6 0 0 ","------+------+------","4 8 0 |0 0 0 |0 9 7 ","7 0 0 |0 0 0 |0 0 5 ","9 3 0 |0 0 0 |0 6 1 ","------+------+------","0 0 8 |3 0 0 |0 5 6 ","0 0 0 |0 0 6 |0 0 0 ","0 0 0 |0 0 7 |4 0 3 "],"message":""}"#
            .as_bytes())
        );
    }

    #[test]
    fn solve_err_puzzle() {
        let resp = TestRequest::with_header("content-type", "application/json")
        .method(http::Method::POST)
        .set_payload(r#"{"puzzle": "X00000600060001070804020005000470000089000340000039000600050709010300020003000004"}"#)
        .run(&solve)
        .unwrap();

        assert!(resp.status().is_success());

        assert_eq!(resp.body(), &Body::from_slice(
            r#"{"status":"fail","data":"","message":"Invalid Grid.  Provide a string of 81 digits with 0 or . for empties."}"#
            .as_bytes())
        );
    }

    #[test]
    fn solve_err_json() {
        let resp = TestRequest::with_header("content-type", "application/json")
                .method(http::Method::POST)
                .set_payload(r#"{"xuzzle": "700000600060001070804020005000470000089000340000039000600050709010300020003000004"}"#)
                .run(&solve)
                .unwrap();

        assert!(resp.status().is_success());

        assert_eq!(resp.body(), &Body::from_slice(
            r#"{"status":"fail","data":"","message":"Invalid Grid.  Provide a string of 81 digits with 0 or . for empties."}"#
            .as_bytes())
        );
    }
}
