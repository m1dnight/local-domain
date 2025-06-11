use aarm_core::delta_proof::{DeltaInstance, DeltaProof, DeltaWitness};
use aarm_core::resource::Resource;
use risc0_zkvm::{Digest, Receipt};
use rustler;
use serde::{Deserialize, Serialize};

//----------------------------------------------------------------------------//
//                                Logic Proof                                 //
//----------------------------------------------------------------------------//

#[derive(Clone, Debug, Deserialize, Serialize, rustler::NifStruct)]
#[module = "Elixir.Zkvm.LogicProof"]
pub struct LogicProof {
    // Receipt contains the proof and the public inputs
    pub receipt: Receipt,
    pub verifying_key: Digest,
}


impl rustler::Encoder for LogicProof {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a>  {
        let data =&** self;
        data.encode(env)
    }
}
//----------------------------------------------------------------------------//
//                                Action                                      //
//----------------------------------------------------------------------------//
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Action {
    pub compliance_units: Vec<Receipt>,
    pub logic_proofs: Vec<LogicProof>,
    pub resource_forwarder_calldata_pairs: Vec<(Resource, ForwarderCalldata)>,
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Delta {
    Witness(DeltaWitness),
    Proof(DeltaProof),
}

//----------------------------------------------------------------------------//
//                                Transaction                                 //
//----------------------------------------------------------------------------//

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub actions: Vec<Action>,
    pub delta_proof: Delta,
}

//----------------------------------------------------------------------------//
//                                Functions                                   //
//----------------------------------------------------------------------------//

#[rustler::nif]
fn testfunc() -> ForwarderCalldata {
    ForwarderCalldata {
        untrusted_forwarder: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        input: vec![],
        output: vec![],
    }
}

rustler::init!("Elixir.Zkvm");
