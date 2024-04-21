use std::env::{args, current_dir, var};
use std::process::exit;
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
    match args.get(0)
        .expect("No argument provided")
        .as_str()
    {
        "tag" => {
            if let Some(arg) = args.get(1) {
               if let Ok(reg_i) = arg.parse::<usize>() {
                    if reg_i < registers.len() {
                        registers[reg_i].push_str(&path);
                        println!("echo \"Pushing into register {reg_i}: {}\"", &registers[reg_i]);
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
                    println!("echo \"Pushing {path} into empty register {i}\"");
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
            if registers.iter().all(|s| s.is_empty()) {
                print!("echo \"All registers are empty\"");
                exit(0);
            }
            if let Some(arg) = args.get(1) {
               if let Ok(reg_i) = arg.parse::<usize>() {
                    if reg_i < registers.len() {
                        if registers[reg_i].is_empty() {
                            println!("echo \"\u{f057} Register {reg_i} is empty\"");
                            exit(1);
                        }
                        println!("cd {}", &registers[reg_i]);
                        println!("echo \"Moving into register {reg_i}: {}\"", &registers[reg_i]);
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
            // cd into the last non-empty register
            for (i,reg) in registers.iter().enumerate().rev() {
                if !reg.is_empty() {
                        println!("cd {}", reg);
                        println!("echo \"Moving into register {i}: {reg}\"");
                        exit(0);
                }
            }
        }
        "list" => {
            if registers.iter().all(|reg| reg.is_empty()) {
                println!("echo \"\u{f51e} Current registers are empty\"")
            } else {
                println!("echo \"\u{f51e} Current registers:\n{}\"", registers
                .iter()
                .enumerate()
                .filter_map(|(i, val) |{
                    if val.is_empty() {
                        None
                    } else {
                        Some(format!("[{}] {}", i, val))
                    }
                }).collect::<Vec<String>>().join("\n"))
            }
        }
        "purge" => {
            println!("unset tagger_registers");
            println!("echo \"\u{f057} Purged registers\"");
        }
        _ => panic!("Invalid argument provided"),
    };
}
