mod ui;
mod solvers;

use std::process::exit;
use ui::play;
use solvers::Solver;


fn main() {
    // breadth_first_solve("/home/pluumenbrownie/rust_projects/rusthour/gameboards/Rushhour9x9_6.csv");
    play_manually("/home/pluumenbrownie/rust_projects/rusthour/gameboards/Rushhour6x6_test.csv");
}

pub fn breadth_first_solve(filename: &str) {
    let solver = Solver::from_file(filename, usize::MAX);
    let _ = solver.breadth_first();
}


pub fn bench_breadth_first() {
    breadth_first_solve("/home/pluumenbrownie/rust_projects/rusthour/gameboards/Rushhour9x9_4.csv");
}


pub fn play_manually(filename: &str) {
    match play(filename) {
        Err(()) => panic!("Aaaaaaa!"),
        Ok(score) => {
            println!("Final score: {score}!");
            exit(0)
        }
    };
}
