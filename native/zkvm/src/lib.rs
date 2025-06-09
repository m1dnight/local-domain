use crate::prover::generate_proof;
use crate::prover::verify_proof;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use rustler;
use serde::{Deserialize, Serialize};
mod prover;

//----------------------------------------------------------------------------//
//                                Logic Proof                                 //
//----------------------------------------------------------------------------//

// #[derive(Clone, Debug, Deserialize, Serialize, rustler::NifStruct)]
// #[module = "Elixir.Zkvm.LogicProof"]
// pub struct LogicProof {
//     // Receipt contains the proof and the public inputs
//     pub receipt: Receipt,
//     pub verifying_key: Digest,
// }

//----------------------------------------------------------------------------//
//                                Action                                      //
//----------------------------------------------------------------------------//
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Action {
//     pub compliance_units: Vec<Receipt>,
//     pub logic_proofs: Vec<LogicProof>,
//     pub resource_forwarder_calldata_pairs: Vec<(Resource, ForwarderCalldata)>,
// }

//----------------------------------------------------------------------------//
//                                Forwarder Call Data                         //
//----------------------------------------------------------------------------//

#[derive(Clone, Debug, Deserialize, Serialize, rustler::NifStruct)]
#[module = "Elixir.Zkvm.ForwarderCalldata"]
pub struct ForwarderCalldata {
    pub untrusted_forwarder: Vec<u8>,
    pub input: Vec<u8>,
    pub output: Vec<u8>,
}

//----------------------------------------------------------------------------//
//                                Action                                      //
//----------------------------------------------------------------------------//

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub enum Delta {
//     Witness(DeltaWitness),
//     Proof(DeltaProof),
// }

//----------------------------------------------------------------------------//
//                                Transaction                                 //
//----------------------------------------------------------------------------//

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Transaction {
//     // pub actions: Vec<Action>,
//     pub delta_proof: Delta,
// }

//----------------------------------------------------------------------------//
//                                Functions                                   //
//----------------------------------------------------------------------------//

#[rustler::nif]
fn prove(a: u64, b: u64) -> String {
    println!("params: {}, {}", a, b);
    let (receipt, _number): (Receipt, u64) = generate_proof(a, b);

    let serialized: String = serde_json::to_string(&receipt).unwrap();
    serialized
}

#[rustler::nif]
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

rustler::init!("Elixir.Zkvm");
