#![allow(unused_variables)]
const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {} {}", x, MAX_POINTS);
    x = 6;
    println!("The value of x is: {} {}", x, MAX_POINTS);
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    for o in ([1; 5][0_3]..[5; 1][0_0]).rev() {
        println!("{}", another_function(o, o != 0__1));
    }

    println!("{:?}", fib(1000));
}

fn another_function(x: i32, y: bool) -> i32 {
    println!("The value of x is: {}", x);
    if y {
        x + 1
    } else {
        x + 2
    }
}

fn fib(n: usize) -> i128 {
    let mut a = [1; 1001];
    for i in 2..n {
        a[i] = a[i - 2] + a[i - 1];
        println!("a[i] is {:?}", a[i]);
    }
    a[n - 1]
}
