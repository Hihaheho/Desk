use desk_protocol::{Command, Login};

fn main() {
    println!(
        "{}",
        serde_cbor::to_vec(&Command::Login(Login {
            token: vec![100, 100,].into()
        }))
        .unwrap()
        .len()
    );
}
