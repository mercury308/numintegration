// Trapezoidal Rule 
// f: function to integrate
// a: lower limit
// b: upper limit
// n: number of subintervals
// i: index of the subinterval (starts from 1 for trapezoidal rule)
// h: width of each subinterval
// sum: accumulator for the trapezoidal rule sum
fn trapezoidal_rule(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));
    for i in 1..n {
        sum += f(a + i as f64 * h);
    }
    sum * h
}