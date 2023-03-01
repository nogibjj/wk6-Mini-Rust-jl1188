# wk6-mini-rust-jl1188:
A Rust Mini Project command-line tool that simplifies file paths. 

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [rust-new-project-template](https://github.com/noahgift/rust-new-project-template)
* [rust-marco-polo-example](https://github.com/noahgift/rust-mlops-template/tree/main/MarcoPolo)

## Usage
<code>cargo run -- simplify --input "/home//foo/"</code>
 
 The command line tool uses the subcommand "<code>simplify</code>" and takes in one argument, a <code>String</code> named "<code>input</code>," as the above examples shows. The result should be outputted as "/home/foo"