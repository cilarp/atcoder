use proconio::input;
use proconio::marker::*;
use std::cmp::*;

fn gcd(n: usize,m: usize) -> usize{
    assert!(n > m);
    if m == 0{
        return n
    }
    gcd(m,n % m)
}

fn lcm(n: usize,m: usize) -> usize{
    assert!(n > m);
    n / gcd(n,m) * m
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let (N,M) = {
        if n < m{
            (m,n)
        }else{
            (n,m)
        }
    };
    let res = lcm(N,M);
    println!("{}",res);
}
