fn main() {
    // Projects can be split into: multiple modules
    // Modules can be split into: multiple files
    // A package can contain multiple binary crates and one optional library crate
    /*
        Module system:
        Packages: Cargo feature that lets you build, test, and share crates
        Crates: A tree of modules that produces a library or executable
        Modules & use: Let you control the organization, scope, and privacy of paths
        Paths: A way of naming an item, such as a struct, function, or module
    */

    println!("7.1.0 - Packages and Crates");
    // Crates are binaries or libraries
    // "crate root" is a source file is the root module of a crate
    // The compiler starts from "crate root"

    // Packages are one or more crates that provide a set of functionality
    // A packages requires a Cargo.toml file
    // The Cargo.toml file describes how to build the contained crates

    // In all precious chapter packages the crate root was src/main.rs
    // Cargo knows this due to a name convention
    // Therefore the enums-project's crate root is has the name enums

    // When working with libraries, the crate root is src/lib.rs

    println!("Cargo passes the crate root file (i.e. src/main.rs or src/lib.rs to rustc, in order to build the binary or library");

    // The cuurent package (until now) only contains src/main.rs:
    // therefore it only contains one crate named packages_and_crates

    // If we add a src/lib.rs, the packages would contain two crates:
    // one binary and one library
    // one package can have multiple binary crates (by placing multiple crates underneath src/bin)
    println!(
        "One package can have one library crate (src/lib.rs) and multiple binary crates (src/bin/)"
    );

    // The semantic logic of a crate is to bundle related functionality
    // example: with the rand crate, functionality for generating random numbers can be imported
    
    // Via namespacing we can control the scope of a crate
    // therefore, we can have the same traits / naming but a crate's functionality will have a "prefix ->" :: 
}
