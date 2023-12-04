mod ui;
// mod solvers;

use std::process::exit;
use ui::{play, list_boards, print_boards};
use ui::solvers::Solver;

use clap::{Args, Parser, Subcommand, ValueEnum};


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Actions>,
}


#[derive(Debug, Subcommand)]
enum Actions {
    /// Print out all found gameboards
    List,
    /// Todo!
    Solve,
    /// Play RustHour manually
    Manual(Manual),
}


#[derive(Args, Debug)]
struct Manual {
    /// The name of the gameboard to solve manually.
    board_name: String,
}


fn main() {
    let cli = Cli::parse();

    if let None = &cli.command {
        panic!("No commands supplied!");
    }

    match &cli.command.unwrap() {
        Actions::List => print_boards(),
        Actions::Manual(input) => {
            let board_list = match list_boards() {
                Ok(result) => result,
                Err(error) => {
                    println!("{}", error);
                    panic!("No valid dir found.");
                }
            };
            
            println!("Searching board {}", input.board_name);
            // let re = Regex::new(r"\d+").unwrap();
            // let Some(size) = re.captures(filename) else {panic!("Regex failed.")};
        }
        _ => {}
    }
    // match cli.command {
    //     Actions::List => list_boards(),
    //     _ => {}
    // }
    // println!("{:?}", cli);
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
