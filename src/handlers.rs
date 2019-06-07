use actix_web::{web::Payload, HttpResponse, error::{Error, PayloadError}};
use futures::{Future, future, Stream};
use serde::{Deserialize, Serialize};
use bytes::Bytes;

mod sudoku;
use sudoku::{Sudoku, PuzzleError};

#[derive(Deserialize)]
struct SudokuRequest {
    puzzle: String
}

#[derive(Serialize)]
struct SolveResponse {
    status: String,
    data: String,
    message: String
}    

#[derive(Serialize)]
struct DisplayResponse {
    status: String,
    data: Vec<String>,
    message: String
}    

fn solve_sudoku(payload: Result<Bytes, PayloadError>) -> impl Future<Item=String, Error=PuzzleError> {
    match payload {
        Ok(body) => {
            let body_content = std::str::from_utf8(&body).unwrap();
            let sudoku_puzzle: Result<SudokuRequest, _> = serde_json::from_str(body_content);
            match sudoku_puzzle {
                Ok(sp) => {
                    let solution = Sudoku::new().solve(&sp.puzzle);
                    match solution {
                        Ok(s)   => future::ok(s),
                        Err(e)  => future::err(e)
                    }
                }
                _ => future::err(PuzzleError::InvalidGrid)
            } 
        } 
        _ => future::err(PuzzleError::InvalidGrid)
    } 
}

fn display_sudoku(payload: Result<Bytes, PayloadError>) -> impl Future<Item=Vec<String>, Error=PuzzleError> {
    match payload {
        Ok(body) => {
            let body_content = std::str::from_utf8(&body).unwrap();
            let sudoku_puzzle: Result<SudokuRequest, _> = serde_json::from_str(&body_content);
            match sudoku_puzzle {
                Ok(sp) => {
                    let grid = Sudoku::display(&sp.puzzle);
                    match grid {
                        Ok(s)   => future::ok(s),
                        Err(e)  => future::err(e)
                    }
                }
                _ => future::err(PuzzleError::InvalidGrid)
            }
        }
        _ => future::err(PuzzleError::InvalidGrid)
    }
}

pub fn solve(body: Payload) -> impl Future<Item = HttpResponse, Error = Error> {
    body.concat2()
        .then(solve_sudoku)
        .then(|solve_result| { 
                    let sudoku_response = 
                        match solve_result {
                            Ok(solution)    => SolveResponse {status: "success".into(), data: solution, message: "".into()},
                            Err(e)          => SolveResponse {status: "fail".into(), data: "".into(), message: format!("{}", e)}
                        };

                    let json_response = serde_json::to_string(&sudoku_response).unwrap();
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(json_response)
                    )
                }
        )
}

pub fn display(body: Payload) -> impl Future<Item = HttpResponse, Error = Error> {
    body.concat2()
        .then(display_sudoku)
        .then(|grid_result| { 
                    let sudoku_response = 
                        match grid_result {
                            Ok(grid)    => DisplayResponse {status: "success".into(), data: grid, message: "".into()},
                            Err(e)      => DisplayResponse {status: "fail".into(), data: Vec::new(), message: format!("{}", e)}
                        };

                    let json_response = serde_json::to_string(&sudoku_response).unwrap();
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(json_response)
                    )
                }
        )
} 

