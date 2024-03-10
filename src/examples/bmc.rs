// Copyright 2023 
// released under BSD 3-Clause License
// author: Gary Hu
use std::env;

use clap::{Parser, ValueEnum};
use libpatron::ir::*;
use libpatron::*;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Hash, Eq, PartialEq, Debug)]
struct PropertyResult {
    index: usize,
    status: String,
}

#[derive(Parser, Debug)]
#[command(name = "bmc")]
#[command(author = "Kevin Laeufer <laeufer@berkeley.edu>")]
#[command(version)]
#[command(about = "Performs bounded model checking on a btor2 file.", long_about = None)]
struct Args {
    #[arg(
        long,
        value_enum,
        default_value = "bitwuzla",
        help = "the SMT solver to use"
    )]
    solver: Solver,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    dump_smt: bool,
    #[arg(value_name = "BTOR2", index = 1)]
    filename: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Solver {
    Bitwuzla,
    Yices2,
}

fn main() {
    // Get the current executable path
    let exe_path = env::current_exe().expect("Failed to get the current executable path");

    // Get the directory containing the executable
    let exe_dir = exe_path.parent().expect("Failed to get the executable's directory");

    // Append '/src' to the executable's directory
    let new_path = exe_dir.join("src");

    // Convert the new path to a String (if you need it as a String)
    let new_path_str = new_path.to_str().expect("Failed to convert path to string");


    // Get the current PATH
    let mut paths = match env::var_os("PATH") {
        Some(paths) => env::split_paths(&paths).collect::<Vec<_>>(),
        None => vec![],
    };

    // Append the new path
    paths.push(new_path.into());

    // Join the paths back together and set the PATH variable for the current process
    let new_paths = env::join_paths(paths).expect("Failed to join paths");
    env::set_var("PATH", &new_paths);

    let args = Args::parse();
    let (mut ctx, sys) = btor2::parse_file(&args.filename).expect("Failed to load btor2 file!");
    if args.verbose {
        println!("Loaded: {}", sys.name);
        println!("{}", sys.serialize_to_str(&ctx));
        println!();
        println!();
    }
    let k_max =20;
    let checker_opts = mc::SmtModelCheckerOptions {
        check_constraints: true,
        check_bad_states_individually: true,
        save_smt_replay: args.dump_smt,
    };
    let solver = match args.solver {
        Solver::Bitwuzla => mc::BITWUZLA_CMD,
        Solver::Yices2 => mc::YICES2_CMD,
    };
    if args.verbose {
        println!(
            "Checking up to {k_max} using {} and the following options:\n{checker_opts:?}",
            solver.name
        );
    }
    let checker = mc::SmtModelChecker::new(solver, checker_opts);
    let res = checker.check(&mut ctx, &sys, k_max).unwrap();

    let mut results_map = HashMap::new();

    for property_result in res.into_iter() {
        match property_result {
            mc::PropertyCheckResult::Unsat(res) => {
                if results_map.get(&(res as usize)) != Some(&"SAT".to_string()) { // if res is UNSAT and previous status is not SAT
                    results_map.insert(res as usize, "UNSAT".to_string());
                }
            }
            mc::PropertyCheckResult::Sat(res, wit) => { // if res is SAT
                results_map.insert(res as usize, "SAT".to_string());
            }
        }
    }

    // sort the results by index
    let mut results: Vec<_> = results_map.into_iter().collect();
    results.sort_by_key(|r| r.0);

    // print the results in `{RESULT, INDEX}` format
    for (index, status) in results {
        println!("{{{}, {}}}", status, index);
    }
    // Print all unique results
    // for result in results_set {
    //     println!("Property Index {}: {}", result.index, result.status);
    // }
}
