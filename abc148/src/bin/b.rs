use proconio::input;
use proconio::marker::*;
use std::cmp::*;

fn main() {
    input! {
        n: u32,
        s: Bytes,
        t: Bytes,
    }
    for i in 0..n{
        print!("{}{}",
            s[i as usize] as char,
            t[i as usize] as char
        );
    }
    println!();
}
