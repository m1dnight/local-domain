use crate::prover::generate_proof;
use crate::prover::verify_proof;
use std::io::Write;

use risc0_zkvm::{Digest, Receipt};

use rustler;

use aarm::action::ForwarderCalldata;
use aarm_core::resource::Resource;
use rustler::{
    nif, Atom, Binary, Decoder, Encoder, Env, Error, NifResult, NifStruct, OwnedBinary, Term,
};

use risc0_zkvm::sha::{Digestible, DIGEST_BYTES, DIGEST_WORDS};
use std::ops::Deref;

mod prover;

#[derive(Debug)]
struct ResourceP {
    pub name: u64,
    pub hash: Digest,
}

impl<'a> Decoder<'a> for ResourceP {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        // For decoding from a map/keyword list
        let map = term.decode::<std::collections::HashMap<String, Term>>()?;

        let name = map.get("name").ok_or(Error::BadArg)?.decode::<u64>()?;

        let hash: Vec<u8> = map
            .get("hash")
            .ok_or(Error::BadArg)?
            .decode::<Binary>()?
            .to_vec();
        let digest: Digest = hash.try_into().expect("Vec must be exactly 8 bytes");

        //
        // let age = map.get("age")
        //     .ok_or(Error::BadArg)?
        //     .decode::<i32>()?;
        //
        // let active = map.get("active")
        //     .ok_or(Error::BadArg)?
        //     .decode::<bool>()?;

        Ok(ResourceP {
            name: name,
            hash: digest,
        })
    }
}

impl Encoder for ResourceP {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        use rustler::types::map::map_new;

        // encode the name
        let map = map_new(env);
        let map = map
            .map_put(Atom::from_str(env, "name").unwrap(), self.name.encode(env))
            .unwrap();

        // encode the Digest
        let digest: Digest = self.hash;
        let digest_bytes = digest.as_bytes();
        let mut digest_bin = OwnedBinary::new(DIGEST_BYTES).expect("allocation failed");
        digest_bin.as_mut_slice().write_all(&digest_bytes).unwrap();
        let map = map.map_put(Atom::from_str(env, "untrusted_forwarder").unwrap(), self.hash.as_words()).unwrap();

        // store the struct name
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
    let resource = ResourceP { name: 123 };

    resource.en
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
