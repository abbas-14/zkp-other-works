use sp1_sdk::{HashableKey, ProverClient};


pub const MAGIC_SQ_ELF: &[u8] = include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");


fn main() {
    sp1_sdk::utils::setup_logger();

    let client = ProverClient::new();

    let (_, vk) = client.setup(MAGIC_SQ_ELF);

    println!("VK: {}", vk.bytes32());
}   
