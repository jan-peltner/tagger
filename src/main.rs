use std::env::{var, current_dir, args};
fn main() {
    let mut stack: Vec<String> = if var("tagger_stack").is_ok() {
            var("tagger_stack")
            .unwrap()
            .split(":")
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_owned())
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    let path = current_dir()
    .expect("Can't access current directory")
    .to_str()
    .expect("Can't convert current directory to string")
    .to_owned();

    match args()
    .skip(1)
    .next()
    .expect("No argument provided")
    .as_str() {
        "tag" => {
            stack.push(path.to_owned());
            println!("export tagger_stack={}", stack.join(":"));
            print!("echo \"Tagged {}\"", path);
        },
        "pop" => {
            if stack.is_empty() {
                print!("echo \"Can't pop empty stack\"");
                std::process::exit(0);
            }
            let popped = stack.pop().unwrap();
            println!("cd {}", &popped);
            println!("export tagger_stack={}", stack.join(":"));
            print!("echo \"Popped {}\"", &popped);
        }
        ,
        _ => panic!("Invalid argument provided")
    };
}
