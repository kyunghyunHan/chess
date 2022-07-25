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

        Some
    }
}
