mod generate;

use generate::generate;

pub fn resolve(args: Vec<String>) {
    if args.len() == 0 {
        println!("{}", "Password module");
        return;
    }

    match args[0].as_str() {
        "gen" | "generate" => generate(args),
        _ => println!("Unknown command: password {}", args[0])
    }
}