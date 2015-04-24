use std::{io, thread};


fn StartGettingCurrency(x: &mut u32) {
    loop {
        println!("init!");
        counter(x);
        println!("{}", x);
        thread::sleep_ms(3000);
    }
}

fn counter(x: &mut u32) {
    *x = *x + 1
}

fn main() {
    let mut x = 0;
    thread::spawn(move || StartGettingCurrency(&mut x));

    thread::spawn(move || StartGettingCurrency(&mut x));
    let mut stdin = io::stdin();
    let input = &mut String::new();

    input.clear();
    stdin.read_line(input);

    println!("Hello, world!");
}
