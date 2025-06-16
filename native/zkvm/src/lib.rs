use crate::prover::generate_proof;
use crate::prover::verify_proof;
use risc0_zkvm::Receipt;
use rustler;

use risc0_zkvm::sha::Digest;

pub mod action;
pub mod constants;
pub mod logic_proof;
pub mod transaction;
pub mod utils;

mod prover;


#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ComplianceInstance {
    pub consumed_nullifier: Digest,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
// #[module = "Elixir.Zkvm.ComplianceInstance"]
pub struct ComplianceInstanceWrapper {
    pub compliance_instance : ComplianceInstance,
}

// impl Encoder for TestWrapper {
//     fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
//         self.test_internal.consumed_nullifier.as_bytes().encode(env)
//     }
// }

// impl<'a> Decoder<'a> for TestWrapper {
//     fn decode(term: Term<'a>) -> NifResult<Self> {
//         let bytes: Vec<u8> = Decoder::decode(term)?;
//         if bytes.len() != 32 {
//             return Err(Error::BadArg);
//         }
//         let mut array = [0u8; 32];
//         array.copy_from_slice(&bytes);
//         Ok(TestWrapper {
//             test_internal: TestInternal {
//                 consumed_nullifier: Digest::from(array),
//             },
//         })
//     }
// }

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

#[derive(Clone, Debug, rustler::NifStruct)]
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
fn testfunc() -> u64 {
    1
}

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
