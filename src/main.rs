use std::{io, thread};
use http_get::http_get::http_getter;


fn StartGettingCurrency(loop_delay: u32, x: &mut u32) {
    loop {
        println!("init!");
        counter(x);
        println!("{}", x);
        thread::sleep_ms(loop_delay);
    }
}

fn counter(x: &mut u32) {
    *x = *x + 1
}

fn main() {
    let mut x = 0;
    thread::spawn(move || StartGettingCurrency(2000, &mut x));

    thread::spawn(move || StartGettingCurrency(4000, &mut x));

    http_getter::test();

    let mut stdin = io::stdin();
    let input = &mut String::new();

    input.clear();
    stdin.read_line(input);

    println!("Hello, world!");
}
