mod args;

use std::{fmt::Display, fs::read_to_string, process::exit, time::Instant};
use termion::{color, style};

fn main() {
    let args = args::parse_args().unwrap_or_else(|_| {
        eprintln!("Error: parsing CLI arguments failed");
        print!("{}", args::HELP);
        std::process::exit(1);
    });

    // day 255 is a magic day number meaning "run all days"
    if args.day == 255 {
        for day in 1..=12 {
            let input = read_to_string(format!("./input/d{}", day));
            if let Ok(input) = input {
                run(day, 1, input.clone());
                run(day, 2, input.clone());
            } else {
                print_part(day, 1, Instant::now(), "no input");
                print_part(day, 2, Instant::now(), "no input");
            }
        }
    } else if (1..=12).contains(&args.day) {
        let input_file = if args.input.is_some() {
            args.input.unwrap()
        } else if args.example {
            format!("./examples/d{}", args.day)
        } else {
            format!("./input/d{}", args.day)
        };

        if let Ok(input) = read_to_string(input_file) {
            // run requested part, or run both parts if no part is requested
            if let Some(part) = args.part {
                run(args.day, part, input);
            } else {
                run(args.day, 1, input.clone());
                run(args.day, 2, input);
            }
        } else {
            eprintln!(
                "Error: input file for day {} is missing or unreadable",
                args.day
            );
        }
    } else {
        eprintln!("Error: DAY must be 1 through 12, or 255 to run all days");
    }
}

fn run(day: u8, part: u8, input: String) {
    if ![1, 2].contains(&part) {
        eprintln!("Error: part must be 1 or 2");
        exit(1);
    }

    let t = Instant::now();
    match (day, part) {
        (1, 1) => {
            let parsed = aoc2025::d1::parse(input);
            let output = aoc2025::d1::part1(parsed);
            print_part(day, part, t, output);
        }
        (1, 2) => {
            let parsed = aoc2025::d1::parse(input);
            let output = aoc2025::d1::part2(parsed);
            print_part(day, part, t, output);
        }
        (2, 1) => {
            let parsed = aoc2025::d2::parse(input);
            let output = aoc2025::d2::part1(parsed);
            print_part(day, part, t, output);
        }
        (2, 2) => {
            let parsed = aoc2025::d2::parse(input);
            let output = aoc2025::d2::part2(parsed);
            print_part(day, part, t, output);
        }
        (3, 1) => {
            let parsed = aoc2025::d3::parse(input);
            let output = aoc2025::d3::part1(parsed);
            print_part(day, part, t, output);
        }
        (3, 2) => {
            let parsed = aoc2025::d3::parse(input);
            let output = aoc2025::d3::part2(parsed);
            print_part(day, part, t, output);
        }
        (4, 1) => {
            let parsed = aoc2025::d4::parse(input);
            let output = aoc2025::d4::part1(parsed);
            print_part(day, part, t, output);
        }
        (4, 2) => {
            let parsed = aoc2025::d4::parse(input);
            let output = aoc2025::d4::part2(parsed);
            print_part(day, part, t, output);
        }
        (5, 1) => {
            let parsed = aoc2025::d5::parse(input);
            let output = aoc2025::d5::part1(parsed);
            print_part(day, part, t, output);
        }
        (5, 2) => {
            let parsed = aoc2025::d5::parse(input);
            let output = aoc2025::d5::part2(parsed);
            print_part(day, part, t, output);
        }
        (6, 1) => {
            let parsed = aoc2025::d6::parse(input);
            let output = aoc2025::d6::part1(parsed);
            print_part(day, part, t, output);
        }
        (6, 2) => {
            let parsed = aoc2025::d6::parse(input);
            let output = aoc2025::d6::part2(parsed);
            print_part(day, part, t, output);
        }
        (7, 1) => {
            let parsed = aoc2025::d7::parse(input);
            let output = aoc2025::d7::part1(parsed);
            print_part(day, part, t, output);
        }
        (7, 2) => {
            let parsed = aoc2025::d7::parse(input);
            let output = aoc2025::d7::part2(parsed);
            print_part(day, part, t, output);
        }
        (8, 1) => {
            let parsed = aoc2025::d8::parse(input);
            let output = aoc2025::d8::part1(parsed);
            print_part(day, part, t, output);
        }
        (8, 2) => {
            let parsed = aoc2025::d8::parse(input);
            let output = aoc2025::d8::part2(parsed);
            print_part(day, part, t, output);
        }
        (9, 1) => {
            let parsed = aoc2025::d9::parse(input);
            let output = aoc2025::d9::part1(parsed);
            print_part(day, part, t, output);
        }
        (9, 2) => {
            let parsed = aoc2025::d9::parse(input);
            let output = aoc2025::d9::part2(parsed);
            print_part(day, part, t, output);
        }
        (10, 1) => {
            let parsed = aoc2025::d10::parse(input);
            let output = aoc2025::d10::part1(parsed);
            print_part(day, part, t, output);
        }
        (10, 2) => {
            let parsed = aoc2025::d10::parse(input);
            let output = aoc2025::d10::part2(parsed);
            print_part(day, part, t, output);
        }
        (11, 1) => {
            let parsed = aoc2025::d11::parse(input);
            let output = aoc2025::d11::part1(parsed);
            print_part(day, part, t, output);
        }
        (11, 2) => {
            let parsed = aoc2025::d11::parse(input);
            let output = aoc2025::d11::part2(parsed);
            print_part(day, part, t, output);
        }
        (12, 1) => {
            let parsed = aoc2025::d12::parse(input);
            let output = aoc2025::d12::part1(parsed);
            print_part(day, part, t, output);
        }
        (12, 2) => {
            let parsed = aoc2025::d12::parse(input);
            let output = aoc2025::d12::part2(parsed);
            print_part(day, part, t, output);
        }
        _ => unimplemented!(),
    }
}

fn print_part<T: Display>(day: u8, part: u8, time: Instant, answer: T) {
    let t = if time.elapsed().as_nanos() > 1_000_000_000 {
        ((time.elapsed().as_millis() as f32) / 1000.0, "s")
    } else if time.elapsed().as_nanos() > 1_000_000 {
        ((time.elapsed().as_micros() as f32) / 1000.0, "ms")
    } else if time.elapsed().as_nanos() > 1_000 {
        ((time.elapsed().as_nanos() as f32) / 1000.0, "μs")
    } else {
        (time.elapsed().as_nanos() as f32, "ns")
    };

    println!(
        "{green}🎄{reset} {blue}d{day}p{part}{reset} {answer} {grey}({time}{time_suf})",
        blue = color::Fg(color::Blue),
        green = color::Fg(color::Green),
        grey = color::Fg(color::LightBlack),
        reset = style::Reset,
        time = t.0,
        time_suf = t.1,
    );
}
