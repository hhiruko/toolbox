mod services;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for (i, arg) in args.iter().enumerate().skip(1) {
        println!("Arg {}: {}", i, arg);
    }

    let vars: Vec<String> = (&args[2..]).to_vec();

    match args[1].as_str() {
        "password" => services::password::resolve(vars),
        _          => println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    }
}