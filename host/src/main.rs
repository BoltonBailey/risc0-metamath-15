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

fn main() {

    println!("Reading metamath file");

    let axiom_file = File::open("theory/mm-benchmarks/simple.mm".clone()).expect("Failed to find file");

    let axiom_file_lines: Vec<String> = BufReader::new(axiom_file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    println!("Serializing metamath file");

    let serialized_file: Vec<u32> = to_vec(&axiom_file_lines).unwrap();

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


    // For example: let env = ExecutorEnv::builder().add_input(&vec).build();
    let env = ExecutorEnv::builder()
        .add_input(&serialized_target_theorem)
        .add_input(&serialized_file)
        .build();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = Executor::from_elf(env, METHOD_NAME_ELF).unwrap();

    println!("Running session");

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    println!("Proving session");

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    let theorem_hash : Digest = from_slice(&receipt.journal).unwrap();

    println!(
        "The metamath prover succeeds, and claims it can prove theorem hash {:?}",
        theorem_hash
    );
    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(METHOD_NAME_ID).unwrap();

    println!(
        "The ZK metamath verifier check succeeded"
    );
}
