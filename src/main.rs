mod services;

use std::env;

fn version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    #[cfg(debug_assertions)]
    {
        for (i, arg) in args.iter().enumerate().skip(1) {
            println!("Arg {}: {}", i, arg);
        }
    }

    if args.len() == 1 {
        version();
        return;
    }

    let vars: Vec<String> = (&args[2..]).to_vec();

    match args[1].as_str() {
        "password" => services::password::resolve(vars),
        "version" | "v" => version(),
        _          => {}
    }
}