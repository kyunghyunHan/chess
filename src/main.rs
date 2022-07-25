extern crate args;
extern crate getopts;

use args::{Args, ArgsError};
use chess::{get_rank, Board, BoardStatus, ChessMove, Color, MoveGen, Piece, ALL_RANKS};
use colored::{ColoredString, Colorize};
use getopts::Occur;
use std::env;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::time::{Duration, Instant};
mod benchmarks;
mod piecesValues;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const TEST_FEN: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
const DEFAULT_DEPTH: i64 = 7;

const PROGRAM_DESC: &'static str = "A good old fashioned Rust chess engine";
const PROGRAM_NAME: &'static str = "Amano";

fn calc_piece_value(pc_inx: usize, sq_idx: usize, color: Option<Color>) -> i64 {
    match color {
        Some(Color::White) => {
            let sq_value = piecesValues::PIECE_SQUARES[pc_inx][sq_idx];
            return -(piecesValues::PIECE_VALS[pc_inx] + sq_value);
        }
        Some(Color::Black) => {
            let sq_value = piecesValues::PIECE_SQUARES[pc_inx][63 - sq_idx];
            return piecesValues::PIECE_VALS[pc_inx] + sq_value;
        }
        None => {
            return 0;
        }
    }
}
fn calc_pieces_value(board: &Board) -> i64 {
    let mut result = 0;
    for pc_idx in 0..6 {
        let pc_type = piecesValues::PIECES[pc_idx];
        let bboard = *board.pieces(pc_type);
        for square in bboard {
            let sq_idx = square.to_index();
            result += calc_piece_value(pc_idx, sq_idx, board.color_on(square));
        }
    }
    result
}
fn main() {
    println!("hello");
}
