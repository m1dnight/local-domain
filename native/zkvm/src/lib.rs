#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    lib::libfn();
    a + b
}

rustler::init!("Elixir.Zkvm");
