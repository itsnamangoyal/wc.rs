use std::{
    env::args,
    io::{self, BufReader, Read},
    ops::Sub,
    path::Path,
    process::exit,
};

struct Args {
    bytes: bool, // -c
    lines: bool, // -l
    words: bool, // -w
    chars: bool, // -m
    file_name: Option<String>,
}

struct Count {
    bytes: i32,
    words: i32,
    lines: i32,
    chars: i32,
}

fn main() {
    let args: Vec<String> = args().skip(1).collect();

    let mut args = Args {
        bytes: args.iter().any(|arg| arg == "-c"),
        lines: args.iter().any(|arg| arg == "-l"),
        words: args.iter().any(|arg| arg == "-w"),
        chars: args.iter().any(|arg| arg == "-m"),
        file_name: args.iter().last().cloned(),
    };

    if !args.bytes && !args.lines && !args.words && !args.chars {
        args.bytes = true;
        args.words = true;
        args.lines = true;
    }

    if args.bytes && args.chars {
        args.chars = false
    }

    let mut count = Count {
        bytes: 0,
        lines: 0,
        words: 0,
        chars: 0,
    };

    let mut last_non_space_byte: i32 = -2;
    let mut curr_byte_count: i32 = -1;

    if args.file_name.is_some() {
        let file_name = args.file_name.clone().unwrap();
        let path = Path::new(&file_name);
        if !path.is_file() {
            eprintln!("{:?} is not a file", file_name);
            exit(1);
        }
        let file = std::fs::File::open(path).unwrap();
        let file_reader = BufReader::new(file);

        for byte in file_reader.bytes() {
            update_count(
                &args,
                &byte.unwrap(),
                &mut count,
                &mut last_non_space_byte,
                &mut curr_byte_count,
            )
        }
    } else {
        let stdin_reader = BufReader::new(io::stdin());

        for byte in stdin_reader.bytes() {
            update_count(
                &args,
                &byte.unwrap(),
                &mut count,
                &mut last_non_space_byte,
                &mut curr_byte_count,
            )
        }
    }

    output_count(&args, &count)
}

fn is_continous_byte(byte: &u8) -> bool {
    (byte & 0b1100_0000) == 0b1000_0000
}

fn update_count(
    args: &Args,
    byte: &u8,
    count: &mut Count,
    last_non_space_byte: &mut i32,
    curr_byte_count: &mut i32,
) {
    *curr_byte_count += 1;

    if args.bytes {
        count.bytes += 1;
    }

    if args.chars {
        if !is_continous_byte(byte) {
            count.chars += 1;
        }
    }

    if args.lines {
        // If a character is a new line add 1 to line count
        if byte.eq(&10) {
            count.lines += 1;
        }
    }

    if args.words {
        if !char::from(byte.clone()).is_whitespace() {
            let last_byte = curr_byte_count.sub(1);
            if last_byte != *last_non_space_byte {
                count.words += 1;
            }

            last_non_space_byte.clone_from(curr_byte_count)
        }
    }
}

fn output_count(args: &Args, count: &Count) {
    let mut output_str = String::from("");

    if args.lines {
        output_str.push_str("    ");
        output_str.push_str(&String::from(count.lines.to_string()));
    }

    if args.words {
        output_str.push_str("   ");
        output_str.push_str(&String::from(count.words.to_string()));
    }

    if args.bytes {
        output_str.push_str("  ");
        output_str.push_str(&String::from(count.bytes.to_string()));
    }

    if args.chars {
        output_str.push_str("  ");
        output_str.push_str(&String::from(count.chars.to_string()));
    }

    if args.file_name.is_some() {
        output_str.push_str(" ");
        output_str.push_str(args.file_name.clone().unwrap().as_ref());
    }

    println!("{}", output_str);
}
