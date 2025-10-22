use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();

    let mut limit_check_ok: bool = false;
    while !limit_check_ok {
        print!("input a number: ");
        io::stdout()
            .flush()
            .unwrap();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        if input.trim_end().parse::<u128>().unwrap() < 35 {
            limit_check_ok = true;
        }
    }

    let input = input.trim_end().parse::<u128>().unwrap();
    let mut product: u128 = input;

    for n in (1..input).rev() {
        product *= n;
    }

    print!("factorial of {input} ({input}!) is: {product}");
}
