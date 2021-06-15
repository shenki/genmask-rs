// Prints out a bitmask with bits set from upper to lower
// Copyright 2021 Joel Stanley <joel@jms.id.au>, IBM Corp.
// SPDX-License-Identifier: GPL-3.0-or-later

use std::env;

// Algorithim from include/linux/bits.h in the Linux kernel
fn genmask(h: u32, l: u32) -> u32 {
    let bits_per_long: u32 = 32;
    let zero: u32 = 0;
    let one: u32 = 1;

    ((!zero) - (one << (l)) + 1) & (!zero >> (bits_per_long - 1 - (h)))
}


fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let lower = match args.pop() {
        Some(val) => val,
        None => panic!("lower not specified"),
    };

    let upper = match args.pop() {
        Some(val) => val,
        None => panic!("Upper not specified"),
    };

    let lower = match lower.parse::<u32>() {
        Ok(n) => n,
        Err(_e) => panic!("{} is not a number", lower),
    };

    let upper = match upper.parse::<u32>() {
        Ok(n) => n,
        Err(_e) => panic!("{} is not a number", upper),
    };

    let mask: u32 = genmask(upper, lower);

    println!("{:#010x} ({:#b})", mask, mask);
}
