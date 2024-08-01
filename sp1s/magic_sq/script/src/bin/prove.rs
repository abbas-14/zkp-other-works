
pub const MAGIC_SQ_ELF: &[u8] = include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

use sp1_sdk::{SP1Stdin, ProverClient};

fn main() {
    sp1_sdk::utils::setup_logger();

    let client  = ProverClient::new();

    println!("Setting up keys..");
    let (pk, vk) = client.setup(MAGIC_SQ_ELF);

    let mut stdin = SP1Stdin::new();
    
    let vals: [u32; 9] = [31, 73, 7, 13, 37, 61, 67, 1, 43];
    stdin.write(&vals);

    let sum = 111;
    stdin.write(&sum);

    println!("Generating proof..");
    let proof = client.prove(&pk, stdin).run().expect("to generate a proof");
    println!("Public values: {:?}", proof.public_values.as_slice());
    
    println!("Verifying proof..");
    client.verify(&proof, &vk).expect("to verify a given proof");
    println!("verification successfull.");

}
