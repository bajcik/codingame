use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let r_1 = input_line.trim().parse::<u32>().unwrap();
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let r_2 = input_line.trim().parse::<u32>().unwrap();

    // some tests
    eprintln!("sumdigits(1)={}", sumdigits(1));
    eprintln!("sumdigits(435)={}", sumdigits(435));
    eprintln!("meeting_point(1,7)={}", meeting_point(1, 7));
    
    println!("{}", meeting_point(r_1, r_2));
}

fn sumdigits(num: u32) -> u32 {
    let mut sum = 0;
    let mut n = num;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    
    sum
}

fn meeting_point(r1: u32, r2: u32) -> u32 {
    let mut n1 = r1;
    let mut n2 = r2;
    
    loop {
        if n1 == n2 { return n1 }
        if n1 < n2 {
            n1 += sumdigits(n1);
        } else {
            n2 += sumdigits(n2);
        }
    }
}

