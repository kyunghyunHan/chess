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

fn calc_board_value(board: &Board) -> i64 {
    let w_move = board.side_to_move() == Color::White;
    let result = match board.status() {
        BoardStatus::Checkmate => {
            if w_move {
                2000
            } else {
                -20000
            }
        }
        BoardStatus::Stalemate => 0,
        BoardStatus::Ongoing => calc_pieces_value(board),
    };
    result
}

fn alpha_beta(
    board: &Board,
    depth: i8,
    is_max: bool,
    alpha: i64,
    beta: i64,
    total: &mut i64,
) -> i64 {
    if (depth == 0) || (board.status() != BoardStatus::Ongoing) {
        *total += 1;
        let val = calc_board_value(board);
        return val;
    }

    let mut alpha = alpha;
    let mut beta = beta;
    if is_max {
        let mut best_val = i64::MIN;
        let moves = MoveGen::new_legal(&board);
        let mut result_board = chess::Board::default();
        for mv in moves {
            board.make_move(mv, &mut result_board);
            let value = alpha_beta(&result_board, depth - 1, false, alpha, beta, total);
            best_val = std::cmp::max(value, best_val);
            alpha = std::cmp::max(alpha, best_val);
            if beta <= alpha {
                break;
            }
        }
        return best_val;
    } else {
        let mut best_val = i64::MAX;
        let moves = MoveGen::new_legal(&board);
        let mut result_board = chess::Board::default();
        for mv in moves {
            board.make_move(mv, &mut result_board);

            let value = alpha_beta(&result_board, depth - 1, true, alpha, beta, total);
            best_val = std::cmp::min(value, best_val);

            beta = std::cmp::min(beta, best_val);
            if beta <= alpha {
                break;
            }
        }
        return best_val;
    }
}
fn show_board(board: Board) {
    for (&rank, lbl) in ALL_RANKS.iter().zip("12345678".chars()) {
        print!("{}", lbl);
        print!(" ");
        for sq in get_rank(rank) {
            let pices = board.piece_on(sq);
        }
    }
}

fn main() {
    println!("hello");
}
