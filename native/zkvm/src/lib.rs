use crate::prover::generate_proof;
use crate::prover::verify_proof;

use risc0_zkvm::Receipt;

use rustler;

use aarm::action::ForwarderCalldata;
use rustler::nif;

mod prover;

//----------------------------------------------------------------------------//
//                                Functions                                   //
//----------------------------------------------------------------------------//

#[nif]
fn testfunc() -> ForwarderCalldata {
    ForwarderCalldata {
        untrusted_forwarder: vec![],
        input: vec![],
        output: vec![],
    }
}

#[nif]
fn prove(a: u64, b: u64) -> String {
    println!("params: {}, {}", a, b);
    let (receipt, _number): (Receipt, u64) = generate_proof(a, b);

    let serialized: String = serde_json::to_string(&receipt).unwrap();
    serialized
}

#[nif]
fn verify(receipt: String) -> bool {
    match serde_json::from_str(&receipt) {
        Ok(r) => {
            return verify_proof(r);
        }
        _ => {
            return false;
        }
    }
}

rustler::init!("Elixir.Anoma.Zkvm");
