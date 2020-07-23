use proconio::input;
use proconio::marker::*;
use std::cmp::*;

fn main() {
    input! {
        n: u128
    }

    if n % 2 == 1{
        println!("0");
        exit();
    }

    let ten: f64 = 10.0;
    let max_times = (ten.log(5.0) * 18.0).ceil() as u32;
    let mut res = 0u128;
    for i in 1..max_times{
        res += n / (2 * 5u128.pow(i));
    }
    println!("{}",res);
}

fn exit(){
    std::process::exit(0);
}