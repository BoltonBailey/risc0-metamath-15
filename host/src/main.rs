// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv,
};
use risc0_zkvm::sha::Digest;

use std::fs::File; 
use std::io::BufReader;
use std::io::BufRead;
use std::cmp;
use std::time::{SystemTime};


fn analyze() {

    println!("Reading metamath file");

    let axiom_file = File::open("theory/set_comp4.mm".clone()).expect("Failed to find file"); // mm-benchmarks/transfer-goal

    let axiom_file_lines: Vec<String> = BufReader::new(axiom_file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut max_lines = 10;
    while max_lines < axiom_file_lines.len() {

        max_lines *= 2;

        let num_lines_to_take = cmp::min(max_lines, axiom_file_lines.len());



        // let mut shortened_axiom_file_lines = axiom_file_lines.copy();

        // for line in &axiom_file_lines[0..num_lines_to_take] {
        //     println!("{}", line);
        // }

        println!("Serializing metamath file");

        let serialized_file: Vec<u32> = to_vec(&axiom_file_lines[0..num_lines_to_take]).unwrap();

        println!("Serializing target theorem");

        let target_theorem: String = "th1".to_string();

        let serialized_target_theorem: Vec<u32> = to_vec(&target_theorem).unwrap();

        // First, we construct an executor environment
        // let env = ExecutorEnv::builder().build();

        // TODO: add guest input to the executor environment using
        // ExecutorEnvBuilder::add_input().
        // To access this method, you'll need to use the alternate construction
        // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
        // done adding input, call ExecutorEnvBuilder::build().

        println!("Creating environment");

        // Default is const DEFAULT_SESSION_LIMIT: usize = 64 * 1024 * 1024; // 64M cycles

        // For example: let env = ExecutorEnv::builder().add_input(&vec).build();
        let env = ExecutorEnv::builder()
            .session_limit(1024 * 1024 * 1024)
            .add_input(&serialized_target_theorem)
            .add_input(&serialized_file)
            .build();

        // Next, we make an executor, loading the (renamed) ELF binary.
        let mut exec = Executor::from_elf(env, METHOD_NAME_ELF).unwrap();

        println!("Running session");
        let mut start_time = SystemTime::now();

        // Run the executor to produce a session.
        let session = exec.run().unwrap();

        match SystemTime::now().duration_since(start_time) {
            Ok(n) => println!(
                "The non-ZK metamath ran the session, taking {} milliseconds for {} lines",
                n.as_millis(),
                num_lines_to_take
            ),
            Err(_) => panic!("SystemTime error"),
        }

        println!("Proving session");
        start_time = SystemTime::now();

        // Prove the session to produce a receipt.
        let receipt = session.prove().unwrap();

        match SystemTime::now().duration_since(start_time) {
            Ok(n) => println!(
                "The ZK metamath ran the session, taking {} milliseconds for {} lines",
                n.as_millis(),
                num_lines_to_take
            ),
            Err(_) => panic!("SystemTime error"),
        }

        println!("Verifying");
        start_time = SystemTime::now();


        let theorem_hash : Digest = from_slice(&receipt.journal).unwrap();
        receipt.verify(METHOD_NAME_ID).unwrap();

        match SystemTime::now().duration_since(start_time) {
            Ok(n) => println!(
                "The ZK metamath verifier succeeds in verifying the proof, taking {} milliseconds for {} lines",
                n.as_millis(),
                num_lines_to_take
            ),
            Err(_) => panic!("SystemTime error"),
        }



    }

}

fn main() {

    analyze()

}
