fn get_fizzbuzz_text(n: u32) -> String {
    match n {
        x if x % 15 == 0 => "fizzbuzz".to_string(),
        x if x % 5 == 0 => "buzz".to_string(),
        x if x % 3 == 0 => "fizz".to_string(),
        x => x.to_string(),
    }
}

macro_rules! fizzbuzz {
    ($n:expr) => {
        let r = get_fizzbuzz_text($n);
        println!("{}", r);
    };
}

macro_rules! fizzbuzz2 {
    ($n: expr, $($nx: expr), +) => {
         let mut r = get_fizzbuzz_text($n);

        $(
            r.push(',');
            r.push_str(&get_fizzbuzz_text($nx));
        )+

        println!("{}", r);
    }
}

fn main() {
    for n in 1..101 {
        fizzbuzz!(n);
    }

    fizzbuzz2!(1, 3, 5, 15, 19, 20, 21, 22);
}
