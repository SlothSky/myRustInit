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

    println!("\n7.2.0 - Modules");
    // use brings a path into scope
    // pub makes items public
    
    // Modules organize code within a crate 
    // Groups code in crates for readability and easy reuse
    // Modules are responsible for code's privacy (public <-> private)
    
    // as example following library crate (restaurant functionatlity) is defined:
    // first part: front of house (what the customers gets)
    // second part: back of house (behind the shelf)
    println!("In order to create a new library: cargo new --lib lib_name");
    // however in our use case we simply use this package and put the lib functions in src/lib.rs

    println!("main.rs and lib.rs are crate roots as they are a module always named crate");
    // Underneath a crate module, there can be the module tree we see in ./lib.rs
    
    // if there is a module underneath another, this module is the child of another and vice versa
    // modules can be siblings, because they have the same parent
    
    // However, all modules (i.e. the module tree is) are underneath the implicit module "crate" 
    // in this case crate is main.rs and lib.rs
}
