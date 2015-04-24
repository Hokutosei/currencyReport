use utils::utils;
use std::{thread};

pub fn test() {
    println!("called!")
}

pub fn start_getting_currency(loop_delay: u32, x: &mut u32) {
    loop {
        println!("init!");
        utils::counter(x);
        println!("{}", x);
        thread::sleep_ms(loop_delay);
    }
}
