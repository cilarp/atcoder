use proconio::input;
use proconio::marker::*;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut count = 0;
    for i in a{
        if i == (count + 1){
            count += 1;
        }
    }

    let res = {
        if count == 0{
            -1
        }else{
            (n - count) as i32
        }
    };
    
    print!("{}",res);
}
