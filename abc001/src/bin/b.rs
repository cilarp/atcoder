
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn main() -> Result<(),Box<dyn std::error::Error>>{
    input!(
        m: u32
    );
    let kilo:u32  = 1000;

    if 70 * kilo < m{
        println!("89");
        return Ok(())
    }

    if 35 * kilo <= m{
        let km: u32 = m / kilo;
        let res = (km - 30) / 5 + 80;
        println!("{}",res);
        return Ok(())
    }

    if 6 * kilo <= m{
        let km = m / kilo;
        let res = km + 50;
        println!("{}",res);
        return Ok(())
    }

    if kilo / 10 <= m{
        let km = m / 100;
        if km < 10{
            println!("0{}",km);
        }
        else{
            println!("{}",km);
        }

        return Ok(())
    }

    println!("00");
    Ok(())
}
