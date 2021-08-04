use algonaut::algod::{v2::Algod, AlgodBuilder};

use crate::provider::Provider;

pub fn algod() -> Algod {
    AlgodBuilder::new()
        .bind("http://127.0.0.1:53630")
        .auth("44d70009a00561fe340b2584a9f2adc6fec6a16322554d44f56bef9e682844b9")
        .build_v2()
        // expect: build returns an error if the URL or token are not provided or have an invalid format,
        // we are passing verified hardcoded values.
        .expect("Couldn't initialize algod")
}

pub fn provider(algod: Algod) -> Provider {
    Provider::new(algod)
}
