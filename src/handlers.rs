use actix_web::{error::Error, web::BytesMut, web::Payload, HttpResponse};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

mod sudoku;
use sudoku::{PuzzleError, Sudoku};

#[derive(Deserialize)]
struct SudokuRequest {
    puzzle: String,
}

#[derive(Serialize)]
struct SolveResponse {
    status: String,
    data: String,
    message: String,
}

#[derive(Serialize)]
struct DisplayResponse {
    status: String,
    data: Vec<String>,
    message: String,
}

fn solve_sudoku(body: &BytesMut) -> Result<String, Box<dyn std::error::Error>> {
    let body_content = std::str::from_utf8(body)?;

    let sudoku_puzzle: Result<SudokuRequest, _> = serde_json::from_str(body_content);
    match sudoku_puzzle {
        Ok(sp) => Ok(Sudoku::new().solve(&sp.puzzle)?),
        _ => Err(PuzzleError::InvalidGrid.into()),
    }
}

fn display_sudoku(body: &BytesMut) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let body_content = std::str::from_utf8(body)?;

    let sudoku_puzzle: Result<SudokuRequest, _> = serde_json::from_str(body_content);
    match sudoku_puzzle {
        Ok(sp) => Ok(Sudoku::display(&sp.puzzle)?),
        _ => Err(PuzzleError::InvalidGrid.into()),
    }
}

pub async fn solve(mut body: Payload) -> Result<HttpResponse, Error> {
    let mut bytes = BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let solve_result = solve_sudoku(&bytes);
    let sudoku_response = match solve_result {
        Ok(solution) => SolveResponse {
            status: "success".into(),
            data: solution,
            message: "".into(),
        },
        Err(e) => SolveResponse {
            status: "fail".into(),
            data: "".into(),
            message: format!("{}", e),
        },
    };

    let json_response = serde_json::to_string(&sudoku_response).unwrap();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response))
}

pub async fn display(mut body: Payload) -> Result<HttpResponse, Error> {
    let mut bytes = BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let grid_result = display_sudoku(&bytes);
    let sudoku_response = match grid_result {
        Ok(grid) => DisplayResponse {
            status: "success".into(),
            data: grid,
            message: "".into(),
        },
        Err(e) => DisplayResponse {
            status: "fail".into(),
            data: Vec::new(),
            message: format!("{}", e),
        },
    };

    let json_response = serde_json::to_string(&sudoku_response).unwrap();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response))
}
