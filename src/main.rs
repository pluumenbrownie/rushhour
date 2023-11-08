mod ui;
// mod solvers;

use std::process::exit;
use ui::{play, list_boards};
use ui::solvers::Solver;

use clap::{Args, Parser, Subcommand, ValueEnum};


#[derive(Parser, Default, Debug)]
struct Cli {
    #[command(Subcommand)]
    action: Actions,
}


#[derive(Debug, Subcommand)]
enum Actions {
    List,
    Manual,
    Solve,
}


fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
    // todo!();
    

    // match cli {
    //     Cli::Manual => play_manually("/home/wessel/Documents/rust_hour/rusthour/gameboards/Rushhour6x6_test.csv"),
    //     Cli::Solve{solver} => {
    //         match solver {
    //             Solvers::BreadthFirst => println!("Breadth First.")
    //         }
    //     }
    //     Cli::List => list_boards(),
    // };
    // breadth_first_solve("/home/wessel/Documents/rust_hour/rusthour/gameboards/Rushhour9x9_4.csv");
    // play_manually("/home/wessel/Documents/rust_hour/rusthour/gameboards/Rushhour6x6_test.csv");
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
