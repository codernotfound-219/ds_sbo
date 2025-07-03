pub mod structures;
pub mod greedy_dp;
pub mod marb_heuristic;
pub mod resources;
pub mod tardiness_calculator;

use std::error::Error;

use resources::problem1::problem1;
use resources::problem2::problem2;
use resources::problem3::problem3;
use structures::BatchSchedule;

#[derive(Debug)]
pub enum Solver {
    Marb,
    GDP,
}

impl Solver {
    fn from_str(s: &str) -> Option<Solver> {
        match s.to_lowercase().as_str() {
            "marb" => Some(Solver::Marb),
            "gdp" => Some(Solver::GDP),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Problem {
    Problem1,
    Problem2,
    Problem3,
}

impl Problem {
    fn from_str(s: &str) -> Option<Problem> {
        match s.to_lowercase().as_str() {
            "problem1" => Some(Problem::Problem1),
            "problem2" => Some(Problem::Problem2),
            "problem3" => Some(Problem::Problem3),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Flag {
    Verbose,
}

pub struct Config {
    pub solver: Solver,
    pub problem: Problem,
    pub flag: Option<Flag>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let solver_str = args.next().ok_or("Could not read solver name")?;
        let problem_str = args.next().ok_or("Could not read problem number")?;

        let flag = match args.next() {
            Some(arg) => {
                match arg.as_str() {
                    "-v" => Some(Flag::Verbose),
                    _ => return Err("Unsupported Flag")
                }
            },
            None => None
        };

        if args.next().is_some() {
            return Err("Unsupported parameters")
        }

        let solver = Solver::from_str(&solver_str).ok_or("Invalid Solver Name")?;
        let problem = Problem::from_str(&problem_str).ok_or("Invalid Problem Name")?;

        Ok ( Config {
            solver,
            problem,
            flag,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let schedule: BatchSchedule = match config.solver {
        Solver::Marb => {
            match config.problem {
                Problem::Problem1 => marb_heuristic::solve::solve(&mut problem1()),
                Problem::Problem2 => marb_heuristic::solve::solve(&mut problem2()),
                Problem::Problem3 => marb_heuristic::solve::solve(&mut problem3()),
            }
        },
        Solver::GDP => {
            match config.problem {
                Problem::Problem1 => greedy_dp::solve::solve(&mut problem1()),
                Problem::Problem2 => greedy_dp::solve::solve(&mut problem2()),
                Problem::Problem3 => greedy_dp::solve::solve(&mut problem3()),
            }
        },
    }?;

    match config.flag {
        Some(Flag::Verbose) => println!("{}", schedule),
        None => ()
    }

    Ok(())
}
