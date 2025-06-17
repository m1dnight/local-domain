use crate::prover::generate_proof;
use crate::prover::verify_proof;

use risc0_zkvm::Receipt;

use rustler;

use aarm::action::ForwarderCalldata;
use aarm_core::resource::Resource;
use rustler::{nif, Atom, Decoder, Encoder, Env, Error, NifResult, NifStruct, Term};

use std::ops::Deref;

mod prover;

#[derive(Debug)]
struct ResourceP {
    pub name: u64,
}

impl<'a> Decoder<'a> for ResourceP {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        // For decoding from a map/keyword list
        let map = term.decode::<std::collections::HashMap<String, Term>>()?;

        let name = map.get("name").ok_or(Error::BadArg)?.decode::<u64>()?;
        //
        // let age = map.get("age")
        //     .ok_or(Error::BadArg)?
        //     .decode::<i32>()?;
        //
        // let active = map.get("active")
        //     .ok_or(Error::BadArg)?
        //     .decode::<bool>()?;

        Ok(ResourceP { name: name })
    }
}

impl Encoder for ResourceP {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        use rustler::types::map::map_new;

        let map = map_new(env);
        let map = map
            .map_put(Atom::from_str(env, "name").unwrap(), self.name.encode(env))
            .unwrap();
        let map = map
            .map_put(
                Atom::from_str(env, "__struct__").unwrap(),
                Atom::from_str(env, "Elixir.Anoma.Zkvm.ForwarderCalldata").unwrap(),
            )
            .unwrap();

        map
    }
}

//----------------------------------------------------------------------------//
//                                Functions                                   //
//----------------------------------------------------------------------------//

#[nif]
fn testfunc() -> ResourceP {
    ResourceP { name: 123 }
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
