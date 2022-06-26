fn main() {
    println!("11.0.0 Writing automated tests");
    // automated tests do not check if the program does everything correctly, but if the 
    // program does what the programmer wants it to do

    println!("\n11.1.0 How to write tests");
    // tests are Rust functions, which perform following actions
    /*
        1. set up required data and/or state
        2. run the code to be tested
        3. assert the expected rules 
    */

    println!("\n11.1.1 Anatomy of a test function");
    // test functions need to be annotated with the #[test] tag on the line before the signature
    // see the adder/lib.rs project for examples 

    // tests fail if something in the test function panics
    // see dummy example for this in the adder/src/lib.rs

    println!("\n11.1.2 Checking results with the assert! macro");
    // see adder/src/lib.rs for example(s)

    println!("\n11.1.3 Testing equality with the assert_eq! and assert_ne! macros");
    // the macros compare if an expected result matches / not matches the actual result
    // check again the adder/src/lib.rs for examples
    println!("The order between left and right in the assert_eq & assert_ne does NOT matter");
    // The values compared are required to have the 'PartialEq' and 'Debug' trait implemented
    // most of the time these can be derived: #[derive(PartialEq, Debug)]

    println!("\n11.1.4 Adding custom failure messages");
    // Error messages can be provided as optional parameters of assert/assert_eq/assert_ne
    // format string (can contain {}) can be passed as second/third parameter
    // an example of this can be found in the adder/src/lib.rs

    println!("\n11.1.5 Checking for panics with should_panic");
    // with the should_panic attribute, we can check if the program reacts correctly to 
    // unexpected inputs, i.e. if the code panics, the test passes
    // NOTE: no exact matching when the code panics (could be another part of the code that panics)
    // therefore: expected should panic → see adder/src/lib.rs

    println!("\n11.1.6 Using Result<T, E> in tests");
    // instead of panicing, the Result<T, E> with an Error returned can be a test input
    // see adder/src/lib.rs for example
    println!("should_panic annotation is not compatible with the Result<T, E> tests");

    println!("\n\n11.2.0 Controlling how tests are run");
    // cargo run → compiling code and running output binary
    // cargo test → compiling code in test mode and running output binary
    /*
        some command line options are for the 'cargo test' tool 
        other command line options are for the resulting (built) and executed test binary
            - different options are separated:
                - cargo test --help (for tool options)
                - cargo test -- --help (for binary options)
    */

    
    println!("\n11.2.1 Running tests in parallel or consecutively");
    /*  
        default → multiple tests are run in parallel, using threads
            • faster execution speed → quicker feedback
            • tests must not depend on each other or on a shared state / environmentojk 
        running tests consecutively
            • limit threads via --test-threads option
            • 'cargo test -- --test-threads=1
    */ 

    println!("\n11.2.2 Showing function output");
    // If a test passes, the test library will not show anything printed to stdout
    // i.e. every println! used in the tested element will be 'suppressed'
    // check the lib.rs for an example
    // IN ORDER TO FORCE THE OUTPUT, ALSO FOR PASSING TEST CASES, RUN:
    // cargo test -- --show-output

    println!("\n11.2.3 Running a subset of tests by name");
    // by default, all the tests are executed
    // running a subset of tests (filtered by name) is possible as well
    /*
        • RUNNING SINGLE TESTS
            - defined by the test function name, e,g,:
            - cargo test greater_than_100_guess_with_expected
        • RUNNING MULTIPLE TESTS
            - all tests functions containing the option in their name will be executed, e.g.:
            - cargo test greater_than_100_gues
            - the containing module's name is also part of the test function's name
        • IGNORING SOME TESTS UNLESS SPECIFICALLY REQUESTED
            - time consuming tests might want to be ignored for most of the time 
            - ignored tests will only be executed via the ignored option:
                - cargo test -- --ignored
                - cargo test -- --include-ignored

            - to the [test] annotation, an additional [ignore] annotation needs to be added 
            - see lib.rs for an example
    */
}






