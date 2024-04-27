use std::env::{args, current_dir, var};
use std::process::exit;

fn are_registers_empty(regs: &Vec<String>) -> bool {
    regs.iter().all(|s| s.is_empty())
}

fn main() {
    let mut registers: Vec<String> = if var("tagger_registers").is_ok() {
        var("tagger_registers")
            .unwrap()
            .split(":")
            .map(|s| s.to_owned())
            .collect()
    } else {
        vec![String::from(""); 10]
    };

    let path = current_dir()
        .expect("Can't access current directory")
        .to_str()
        .expect("Can't convert current directory to string")
        .to_owned();
    let args: Vec<String> = args().skip(1).collect();
    if args.len() > 2 {
        println!("echo \"Too many arguments provided\"");
        exit(1);
    }
    match args.get(0).expect("No argument provided").as_str() {
        "tag" => {
            if let Some(arg) = args.get(1) {
                if let Ok(reg_i) = arg.parse::<usize>() {
                    if reg_i < registers.len() {
                        let register_is_empty = registers[reg_i].is_empty();
                        registers[reg_i] = path.to_owned();
                        if register_is_empty {
                            println!(
                                "echo \"Writing to empty register {reg_i}: {}\"",
                                &registers[reg_i]
                            );
                        } else {
                            println!(
                                "echo \"Overwriting register {reg_i}: {}\"",
                                &registers[reg_i]
                            );
                        }
                        println!("export tagger_registers={}", registers.join(":"));
                        exit(0);
                    } else {
                        println!("echo \"Index out of bounds\"");
                        exit(1);
                    }
                } else {
                    println!("echo \"Can't parse argument {arg} as index\"");
                    exit(1);
                }
            }
            let mut found_empty = false;
            for (i, reg) in registers.iter_mut().enumerate() {
                if reg.is_empty() {
                    found_empty = true;
                    reg.push_str(&path);
                    println!("echo \"Writing to empty register {i}: {path}\"");
                    break;
                }
            }
            if !found_empty {
                println!("echo \"Overwriting last register with {path}\"");
                let last_i = registers.len() - 1;
                registers[last_i] = path;
            }
            println!("export tagger_registers={}", registers.join(":"));
        }
        "move" => {
            if are_registers_empty(&registers) {
                print!("echo \"All registers are empty\"");
                exit(0);
            };
            if let Some(arg) = args.get(1) {
                if let Ok(reg_i) = arg.parse::<usize>() {
                    if reg_i < registers.len() {
                        if registers[reg_i].is_empty() {
                            println!("echo \"\u{f057} Register {reg_i} is empty\"");
                            exit(1);
                        }
                        println!("cd {}", &registers[reg_i]);
                        println!(
                            "echo \"Moving into register {reg_i}: {}\"",
                            &registers[reg_i]
                        );
                        exit(0);
                    }
                    println!("echo \"Index out of bounds\"");
                    exit(1);
                }
                println!("echo \"Can't parse argument {arg} as index\"");
                exit(1);
            }
            // cd into the last non-empty register
            for (i, reg) in registers.iter().enumerate().rev() {
                if !reg.is_empty() {
                    println!("cd {}", reg);
                    println!("echo \"Moving into register {i}: {reg}\"");
                    exit(0);
                }
            }
        }
        "echo" => {
            if are_registers_empty(&registers) {
                print!("echo \"All registers are empty\"");
                exit(0);
            };
            if let Some(arg) = args.get(1) {
                if let Ok(reg_i) = arg.parse::<usize>() {
                    if reg_i < registers.len() {
                        if registers[reg_i].is_empty() {
                            println!("\u{f057} Register {reg_i} is empty");
                            exit(1);
                        }
                        println!("{}", &registers[reg_i]);
                        exit(0);
                    }
                    println!("Index out of bounds");
                    exit(1);
                }
                println!("Can't parse argument {arg} as index");
                exit(1);
            }
            println!("Provide an index to echo");
        }
        "list" => {
            if are_registers_empty(&registers) {
                print!("All registers are empty");
                exit(0);
            };
            println!(
                "\u{f51e} Current registers:\n{}",
                registers
                    .iter()
                    .enumerate()
                    .filter_map(|(i, val)| {
                        if val.is_empty() {
                            None
                        } else {
                            Some(format!("[{}] {}", i, val))
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
        "purge" => {
            println!("unset tagger_registers");
            println!("echo \"\u{f057} Purging registers\"");
        }
        _ => panic!("Invalid argument provided"),
    };
}
