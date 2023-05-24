use std::io;

fn main() {
    let prog = "+++++.>,.[-].".as_bytes();
    let mut i = 0;
    let mut stack = Vec::new();
    let mut mem = [0_u8; 1000000];
    let mut ptr = 0;
    loop {
        if prog[i] == '[' as u8 {
            stack.push(i);
        } else if prog[i] == ']' as u8 {
            if mem[ptr] != 0 {
                i = stack.pop().unwrap();
                stack.push(i);
            } else {
                stack.pop();
            }
        } else if prog[i] == '<' as u8 {
            ptr -= 1;
        } else if prog[i] == '>' as u8 {
            ptr += 1;
        } else if prog[i] == '+' as u8 {
            mem[ptr] += 1;
        } else if prog[i] == '-' as u8 {
            mem[ptr] -= 1;
        } else if prog[i] == '.' as u8 {
            println!("{}", mem[ptr]);
        } else if prog[i] == ',' as u8 {
            println!("input: ");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            let res = buf.chars().nth(0).unwrap() as u8;
            mem[ptr] = res;
        }
        i += 1;
        if i >= prog.len() {
            break;
        }
    }
    println!("Done");
}
