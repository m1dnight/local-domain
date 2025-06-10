// // pub const TEST_ZKVM_GUEST_ELF: &[u8] = include_bytes!("/Users/christophe/Documents/Code/rust/zkvm/zk-vm/root_project/target/riscv-guest/methods/test_zkvm_guest/riscv32im-risc0-zkvm-elf/release/test_zkvm_guest.bin");
// // pub const TEST_ZKVM_GUEST_PATH: &str = "/Users/christophe/Documents/Code/rust/zkvm/zk-vm/root_project/target/riscv-guest/methods/test_zkvm_guest/riscv32im-risc0-zkvm-elf/release/test_zkvm_guest.bin";
// // pub const TEST_ZKVM_GUEST_ID: [u32; 8] = [702183157, 4010177511, 136362028, 3798872529, 3886349210, 4002620288, 1909728097, 2731395966];

// pub const TEST_ZKVM_GUEST_ELF: &[u8] =
//     include_bytes!("/Users/christophe/Documents/Work/local-domain/native/zkvm/test_zkvm_guest.bin");
// pub const TEST_ZKVM_GUEST_PATH: &str =
//     "/Users/christophe/Documents/Work/local-domain/native/zkvm/test_zkvm_guest.bin";
// pub const TEST_ZKVM_GUEST_ID: [u32; 8] = [
//     702183157, 4010177511, 136362028, 3798872529, 3886349210, 4002620288, 1909728097, 2731395966,
// ];

// use methods::{
//  TEST_ZKVM_GUEST_ELF, TEST_ZKVM_GUEST_ID
// };

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    lib::libfn();
    a + b
}

rustler::init!("Elixir.Zkvm");

// pub fn libfn() -> risc0_zkvm::Receipt {
//     // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
//     tracing_subscriber::fmt()
//         .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
//         .init();

//     // An executor environment describes the configurations for the zkVM
//     // including program inputs.
//     // A default ExecutorEnv can be created like so:
//     // `let env = ExecutorEnv::builder().build().unwrap();`
//     // However, this `env` does not have any inputs.
//     //
//     // To add guest input to the executor environment, use
//     // ExecutorEnvBuilder::write().
//     // To access this method, you'll need to use ExecutorEnv::builder(), which
//     // creates an ExecutorEnvBuilder. When you're done adding input, call
//     // ExecutorEnvBuilder::build().

//     // For example:
//     let input: u32 = 15 * u32::pow(2, 27) + 1;
//     let env = ExecutorEnv::builder()
//         .write(&input)
//         .unwrap()
//         .build()
//         .unwrap();

//     // Obtain the default prover.
//     let prover = default_prover();

//     println!("here");
// println!("{:?}", TEST_ZKVM_GUEST_ELF);


//     // Proof information by proving the specified ELF binary.
//     // This struct contains the receipt along with statistics about execution of the guest
//     let prove_info = prover.prove(env, TEST_ZKVM_GUEST_ELF).unwrap();

//     println!("now here");

//     // extract the receipt.
//     let receipt = prove_info.receipt;
//     receipt

//     // // TODO: Implement code for retrieving receipt journal here.

//     // // For example:
//     // let output: u32 = receipt.journal.decode().unwrap();
//     // println!("{}", output);

//     // // The receipt was verified at the end of proving, but the below code is an
//     // // example of how someone else could verify this receipt.
//     // receipt
//     //     .verify(TEST_ZKVM_GUEST_ID)
//     //     .unwrap();
// }
