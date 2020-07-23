use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let res = 6 - a - b;
    println!("{}",res);
}
