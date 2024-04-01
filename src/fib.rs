fn fib(n: u32) -> u32 {
    if n <= 2 {
        // The base case.
        return 1;
    } else {
        // The recursive case.
        return fib(n - 1) + fib(n - 2);
    }
}

fn fib2(n: u32) -> u32 {
    if n < 2 {
        return 1;
    }
    let mut a0 = 0;
    let mut a1 = 1;
    for _ in 2..=n {
        (a0, a1) = (a1, a0 + a1);
    }
    a1
}

fn main() {
    let n = 20;
    println!("fib(n) = {}, {}", fib(n), fib2(n));
}
