// Left Riemann Sum 
// f: function to integrate
// a: lower limit
// b: upper limit
// n: number of subintervals
// i: index of the subinterval (starts from 0 for left Riemann sum)
// h: width of each subinterval
// sum: accumulator for the Riemann sum

use std::io;
fn main(){
    println!("Enter the lower limit a:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = a.trim().parse().expect("Please enter a number");

    println!("Enter the upper limit b:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: f64 = b.trim().parse().expect("Please enter a number");

    println!("Enter the number of subintervals n:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a number");
    let result = left_rs(|x| x*x, a, b, n);
    println!("The left Riemann sum is: {}", result);
}
fn left_rs(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.0;
    for i in 0..n {
        sum += f(a + i as f64 * h);
    }
    sum * h
}

