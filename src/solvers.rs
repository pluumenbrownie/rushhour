
// #[path ="board.rs"]
pub mod board;
use std::{collections::HashSet, mem};

use board::Board;
use indicatif::{ProgressIterator, ProgressStyle};
use regex::Regex;


pub struct Solver {
    board: Board,
    max_depth: usize,
}

impl Solver {
    pub fn from_file(filename: &str, max_depth: usize) -> Solver {
        let re = Regex::new(r"\d+").unwrap();
        let Some(size) = re.captures(filename) else {panic!("Regex failed.")};
        let mut board = Board::new(size[0].parse::<u8>().expect("Parsing failed."));

        board.fill(filename);
        Solver {
            board, 
            max_depth
        }
    }


    fn set_max_depth(&mut self, new_depth: usize) {
        self.max_depth = new_depth;
    }


    pub fn breadth_first(&self) -> Result<(), ()> {
        let board = self.board.clone();
        let mut depth_counter = 0usize;
        let mut archive = HashSet::new();
        archive.insert(board.get_hash());
        let mut current_generation: Vec<Board> = vec![board];
        let mut next_generation: Vec<Board> = vec![];

        while depth_counter < self.max_depth {
            next_generation.clear();

            let iterator_with_progress_bar = current_generation.iter()
                .progress_with_style(ProgressStyle::with_template(
                    "{msg} [{elapsed_precise}] {wide_bar} {pos}/{len} "
                ).expect("template failed."))
                .with_message(format!("Depth: {}", depth_counter)
            );

            for state in iterator_with_progress_bar {
                let possible_moves = state.possible_moves()?;

                for vehicle_move in possible_moves {
                    let mut new_state = state.clone();
                    new_state.move_vehicle(vehicle_move);
                    if new_state.is_won()? {
                        new_state.export("results/solution.csv");
                        return Ok(());
                    }
                    if archive.insert(new_state.get_hash()) {
                        next_generation.push(new_state);
                    }
                }
            }

            mem::swap(&mut current_generation, &mut next_generation);
            depth_counter += 1;
        }

        Ok(())
    }
}