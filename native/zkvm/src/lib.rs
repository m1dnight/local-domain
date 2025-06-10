mod foo;

use rustler;
#[derive(Debug, rustler::NifStruct)]
#[module = "Elixir.Zkvm.Action"]
pub struct Action {
    compliance_units: Vec<u8>,
}

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    lib::libfn();
    a + b
}

#[rustler::nif]
fn mkstruct() -> Action {
    Action {
        compliance_units: vec![],
    }
}

rustler::init!("Elixir.Zkvm");
