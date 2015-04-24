extern crate currencyReport;

use std::{io, thread};
use currencyReport::http_get::http_getter;


fn main() {
    let mut x = 0;
    thread::spawn(move || http_getter::start_getting_currency(2000, &mut x));

    thread::spawn(move || http_getter::start_getting_currency(4000, &mut x));

    http_getter::test();

    let mut stdin = io::stdin();
    let input = &mut String::new();

    input.clear();
    stdin.read_line(input);

    println!("Hello, world! {}", input);
}
