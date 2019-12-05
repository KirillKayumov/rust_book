use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Wrong");

    let n: i32 = n.trim().parse().expect("Wrong");

    println!("{}", fib1(n));
}

fn fib1(n: i32) -> i128 {
    let mut q = 0;
    let mut w = 1;
    let mut i = 1;
    let mut res: i128 = w;

    while i < n {
        res = q + w;
        q = w;
        w = res;

        i = i + 1;
    }

    res
}

fn fib2(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    }

    fib2(n - 1) + fib2(n - 2)
}
