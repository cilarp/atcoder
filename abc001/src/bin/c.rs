
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

fn main() {
    input!(
        deg: f64,
        dis: f64,
    );

    let w = cacl_w(dis);
    let dir = {
        if w == 0{
            "C".to_string()
        }else{
            calc_deg(deg)
        }
    };
    println!("{} {}",dir,w);
}

fn cacl_w(dis: f64) -> usize{
    let mut ms = dis / 60.0;
    ms *= 10.0;
    ms = ms.round();
    ms /= 10.0;

    if ms <= 0.2{
        return 0
    }

    if ms <= 1.5{
        return 1
    }

    if ms <= 3.3{
        return 2
    }

    if ms <= 5.4{
        return 3
    }

    if ms <= 7.9{
        return 4
    }

    if ms <= 10.7{
        return 5
    }

    if ms <= 13.8{
        return 6
    }

    if ms <= 17.1{
        return 7
    }

    if ms <= 20.7{
        return 8
    }

    if ms <= 24.4{
        return 9
    }

    if ms <= 28.4{
        return 10
    }

    if ms <= 32.6{
        return 11
    }

    12
}

fn calc_deg(deg: f64) -> String{
    let ten = 10.0;
    if deg < 11.25 * ten{
        return "N".to_string()
    }

    if deg < 33.75 * ten{
        return "NNE".to_string()
    }

    if deg < 56.25 * ten{
        return "NE".to_string()
    }

    if deg < 78.75 * ten{
        return "ENE".to_string()
    }

    if deg < 101.25 * ten{
        return "E".to_string()
    }

    if deg < 123.75 * ten{
        return "ESE".to_string()
    }

    if deg < 146.25 * ten{
        return "SE".to_string()
    }

    if deg < 168.75 * ten{
        return "SSE".to_string()
    }

    if deg < 191.25 * ten{
        return "S".to_string()
    }

    if deg < 213.75 * ten{
        return "SSW".to_string()
    }

    if deg < 236.25 * ten{
        return "SW".to_string()
    }

    if deg < 258.75 * ten{
        return "WSW".to_string()
    }

    if deg < 281.25 * ten{
        return "W".to_string()
    }

    if deg < 303.75 * ten{
        return "WNW".to_string()
    }

    if deg < 326.25 * ten{
        return "NW".to_string()
    }

    if deg < 348.75 * ten{
        return "NNW".to_string()
    }

    "N".to_string()
}
