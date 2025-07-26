use rand::Rng;

pub fn generate(args: Vec<String>) {
    let mut length: i8 = 7;
    let mut count: i8 = 1;

    for (i, arg) in args.iter().enumerate().skip(1) {
        if let Ok(integer) = arg.parse::<i8>() {
            match i {
                1 => length = integer,
                2 => count = integer,
                _ => {}
            }
        } else if arg.contains('=') {
            let parts: Vec<&str> = arg.splitn(2, '=').collect();
            if parts.len() != 2 {
                continue;
            }

            match parts[0] {
                "l" | "len" | "length" => {
                    if let Ok(l) = parts[1].parse::<i8>() {
                        length = l;
                    }
                },
                "c" | "count" => {
                    if let Ok(c) = parts[1].parse::<i8>() {
                        count = c;
                    }
                },
                _ => {}
            }
        }
    }

    print_passwords(length, count);
}

fn print_passwords(length: i8, count:i8) {
    for _ in 0..count {
        println!("{}", generate_password(length));
    }
}

fn generate_password(length: i8) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}|;:,.<>?".chars().collect();
    let last_index: usize = chars.len() - 1;
    let mut password_chars: Vec<char> = vec![];
    let mut rng = rand::rng();
    for _ in 0..length {
        password_chars.push(chars[rng.random_range(1..=last_index)]);
    }
    return password_chars.iter().collect();
}