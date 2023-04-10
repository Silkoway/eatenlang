use std::{env, fs};
use rand::Rng;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(argv[1].clone())
        .expect("Should have been able to read the file");

    let mut stack: Vec<i128> = vec![];

    let tokens: Vec<&str> = contents.split("").collect();

    let mut i = 0;

    while i < tokens.len() {
        let tok = tokens[i];
        match tok {
            "6" => {
                stack.push(69420);
            },
            "." => {
                println!("{}", stack.pop().expect("no items on stack: exited").clone());
            },
            "h" => {
                let top = stack.pop().expect("no items on stack: exited").clone();
                if top == 727 {
                    println!("Hello, world!");
                }
            },
            ":" => {
                let mut rng = rand::thread_rng();
                let rand: i32 = rng.gen_range(0..100);
                let top = stack.pop().expect("no items on stack: exited").clone();
                if rand >= 42 {
                    print!("{}", top as u8 as char);
                } else {
                    stack.push(top);
                    stack.push(1);
                }

            },
            "+" => {
                let top = stack.pop().expect("no items on stack: exited").clone();
                stack.push(top+1);
            },
            "r" => {
                let top = stack.pop().expect("no items on stack: exited").clone();
                i = top.clone() as usize;
            },
            "^" => {
                let top = stack.pop().expect("no items on stack: exited").clone();
                if top > 365 {
                    i+=1;
                }
            },
            _ => {}
        }
        i += 1;
    }

}
