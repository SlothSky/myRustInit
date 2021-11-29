use std::panic;

fn main() {
    println!("9.1.0 - Unrecoverable errors with panic!");
    // panic unwinding: Rust cleans up stack (takes longer & binary us bugger)
    // panic abort: Rust immediately stops the program (clean up by os & smaller image)
    // default is unwinding, otherwise see Cargo.toml
    /*
        panic!("crash and burn"); // -> this line calls the panic macro explicetly
    */

    let v = vec![1, 2, 3];
    v[99]; // -> this line calls the panic macro implicetly 
    // in order to know which functions called the macro, backtrace can be used:
    // backtraces are used via environment variables - set RUST_BACKTRACE != 0
    // RUST_BACKTRACE=1 cargo run

    // In order to get backtraces, debug symbols must be enabled:
    // default of cargo build & cargo run w/o --release
} 
