use std::env;
use std::process::{Command, ExitStatus};

fn print_help() {
    println!("Usage: argswap [options] [--] <command> [args...]");
    println!();
    println!("Options:");
    println!("  -h, --help           Show this help message");
    println!(
        "  -i, --input <idx>    Specify input source indices (supports negative indices like -1)"
    );
    println!(
        "  -o, --output <idx>   Specify output destination indices (supports negative indices like -1)"
    );
    println!("  -d, --drop <idx>     Drop specified indices (supports negative indices like -1)");
    println!(
        "  -s, --swap <idx>     Swap adjacent indices (idx and idx+1) (supports negative indices like -1)"
    );
    println!();
    println!("Note: Indices can be comma-separated or ranges (e.g., 0-2, -3--1).");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        std::process::exit(0);
    }

    let dash_dash_pos = args.iter().position(|r| r == "--");

    let (option_args, target_args) = match dash_dash_pos {
        Some(pos) => (&args[1..pos], &args[pos + 1..]),
        None => {
            let mut split_idx = args.len();
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "-i" | "--input" | "-o" | "--output" | "-d" | "--drop" | "-s" | "--swap" => {
                        i += 2;
                    }
                    _ => {
                        split_idx = i;
                        break;
                    }
                }
            }
            (&args[1..split_idx], &args[split_idx..])
        }
    };

    if target_args.is_empty() {
        eprintln!("Error: No target command specified.");
        std::process::exit(1);
    }

    let total_len = target_args.len();
    let max_index = total_len - 1;

    let mut input_str: Option<&str> = None;
    let mut output_str: Option<&str> = None;
    let mut drop_str: Option<&str> = None;
    let mut swap_str: Option<&str> = None;

    let mut i = 0;
    while i < option_args.len() {
        match option_args[i].as_str() {
            "-i" | "--input" => {
                if i + 1 < option_args.len() {
                    input_str = Some(&option_args[i + 1]);
                    i += 2;
                } else {
                    missing_arg("-i");
                }
            }
            "-o" | "--output" => {
                if i + 1 < option_args.len() {
                    output_str = Some(&option_args[i + 1]);
                    i += 2;
                } else {
                    missing_arg("-o");
                }
            }
            "-d" | "--drop" => {
                if i + 1 < option_args.len() {
                    drop_str = Some(&option_args[i + 1]);
                    i += 2;
                } else {
                    missing_arg("-d");
                }
            }
            "-s" | "--swap" => {
                if i + 1 < option_args.len() {
                    swap_str = Some(&option_args[i + 1]);
                    i += 2;
                } else {
                    missing_arg("-s");
                }
            }
            _ => {
                eprintln!("Error: Unknown option '{}'", option_args[i]);
                std::process::exit(1);
            }
        }
    }

    let mut indexed_args: Vec<(usize, String)> = target_args
        .iter()
        .enumerate()
        .map(|(idx, val)| (idx, val.clone()))
        .collect();

    if let Some(d_val) = drop_str {
        let drop_indices = parse_indices(d_val, total_len);

        for &idx in &drop_indices {
            if idx > max_index {
                eprintln!(
                    "Error: Drop index {} is out of bounds (max: {}).",
                    idx, max_index
                );
                std::process::exit(1);
            }
        }

        indexed_args.retain(|(idx, _)| !drop_indices.contains(idx));
    }

    if let Some(s_val) = swap_str {
        let swap_base_indices = parse_indices(s_val, total_len);

        for &base in &swap_base_indices {
            if base >= max_index {
                eprintln!(
                    "Error: Cannot swap index {} and {}. Out of bounds (max: {}).",
                    base,
                    base + 1,
                    max_index
                );
                std::process::exit(1);
            }
        }

        for base in swap_base_indices {
            let pos1 = indexed_args.iter().position(|(idx, _)| *idx == base);
            let pos2 = indexed_args.iter().position(|(idx, _)| *idx == base + 1);
            if let (Some(p1), Some(p2)) = (pos1, pos2) {
                indexed_args.swap(p1, p2);
            } else {
                eprintln!("Error: Swap target elements were already dropped.");
                std::process::exit(1);
            }
        }
    }

    if let (Some(i_val), Some(o_val)) = (input_str, output_str) {
        let in_indices = parse_indices(i_val, total_len);
        let out_indices = parse_indices(o_val, total_len);

        if in_indices.len() != out_indices.len() {
            eprintln!("Error: The number of elements in --input and --output must match.");
            std::process::exit(1);
        }

        for &idx in &in_indices {
            if idx > max_index {
                eprintln!(
                    "Error: Input index {} is out of bounds (max: {}).",
                    idx, max_index
                );
                std::process::exit(1);
            }
        }

        for &idx in &out_indices {
            if idx > max_index {
                eprintln!(
                    "Error: Output index {} is out of bounds (max: {}).",
                    idx, max_index
                );
                std::process::exit(1);
            }
        }

        let mut target_positions = Vec::new();
        for &in_idx in &in_indices {
            if let Some(pos) = indexed_args.iter().position(|(idx, _)| *idx == in_idx) {
                target_positions.push(pos);
            } else {
                eprintln!(
                    "Error: Input index {} was already dropped by previous options.",
                    in_idx
                );
                std::process::exit(1);
            }
        }

        let mut moved_items = Vec::new();
        for &pos in &target_positions {
            moved_items.push(indexed_args[pos].clone());
        }

        for (k, &out_idx) in out_indices.iter().enumerate() {
            if let Some(pos) = indexed_args.iter().position(|(idx, _)| *idx == out_idx) {
                indexed_args[pos] = moved_items[k].clone();
            } else {
                eprintln!(
                    "Error: Output destination index {} was already dropped.",
                    out_idx
                );
                std::process::exit(1);
            }
        }
    } else if input_str.is_some() || output_str.is_some() {
        eprintln!("Error: Both --input and --output must be specified together.");
        std::process::exit(1);
    }

    let final_args: Vec<String> = indexed_args.into_iter().map(|(_, val)| val).collect();
    if final_args.is_empty() {
        eprintln!("Error: No arguments left to execute.");
        std::process::exit(1);
    }

    let command_name = &final_args[0];
    let command_args = &final_args[1..];

    let status: ExitStatus = Command::new(command_name)
        .args(command_args)
        .status()
        .unwrap_or_else(|err| {
            eprintln!(
                "Error: Failed to execute command '{}': {}",
                command_name, err
            );
            std::process::exit(1);
        });

    if let Some(code) = status.code() {
        std::process::exit(code);
    }
}

fn missing_arg(opt: &str) {
    eprintln!("Error: Missing value for option '{}'", opt);
    std::process::exit(1);
}

fn parse_single_index(s: &str, total_len: usize) -> Option<usize> {
    let s = s.trim();
    if s.starts_with('-') {
        if let Ok(val) = s.parse::<isize>() {
            let idx = total_len as isize + val;
            if idx >= 0 {
                return Some(idx as usize);
            }
        }
        None
    } else {
        s.parse::<usize>().ok()
    }
}

fn parse_indices(s: &str, total_len: usize) -> Vec<usize> {
    let mut indices = Vec::new();
    for part in s.split(',') {
        let part = part.trim();
        if let Some(dash_pos) = part.find('-') {
            if dash_pos > 0
                && !part.as_bytes()[dash_pos - 1].is_ascii_digit()
                && part.as_bytes()[dash_pos - 1] != b' '
            {
                if let Some(idx) = parse_single_index(part, total_len) {
                    indices.push(idx);
                }
                continue;
            }
            let (left, right) = if dash_pos == 0 {
                if let Some(next_dash) = part[1..].find('-') {
                    let actual_dash = next_dash + 1;
                    (&part[..actual_dash], &part[actual_dash + 1..])
                } else {
                    if let Some(idx) = parse_single_index(part, total_len) {
                        indices.push(idx);
                    }
                    continue;
                }
            } else {
                (&part[..dash_pos], &part[dash_pos + 1..])
            };
            if let (Some(start), Some(end)) = (
                parse_single_index(left, total_len),
                parse_single_index(right, total_len),
            ) {
                if start <= end {
                    for idx in start..=end {
                        indices.push(idx);
                    }
                } else {
                    for idx in (end..=start).rev() {
                        indices.push(idx);
                    }
                }
            }
        } else if let Some(idx) = parse_single_index(part, total_len) {
            indices.push(idx);
        }
    }
    indices
}
