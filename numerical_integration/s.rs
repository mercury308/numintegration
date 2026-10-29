// Simpson's Rule 
// f: function to integrate
// a: lower limit
// b: upper limit
// n: number of subintervals (must be even)
// i: index of the subinterval (starts from 1 for Simpson's rule)
// h: width of each subinterval
// sum: accumulator for the Simpson's rule sum
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

    println!("Enter the number of subintervals n (must be even):");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a number");

    let result = simpsons_rule(|x| x*x, a, b, n);
    println!("The Simpson's rule sum is: {}", result);
}
fn simpsons_rule(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n {
        if i % 2 == 0 {
            sum += 2.0 * f(a + i as f64 * h);
        } else {
            sum += 4.0 * f(a + i as f64 * h);
        }
    }
    sum * h / 3.0
}