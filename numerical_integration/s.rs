// Simpson's Rule 
// f: function to integrate
// a: lower limit
// b: upper limit
// n: number of subintervals (must be even)
// i: index of the subinterval (starts from 1 for Simpson's rule)
// h: width of each subinterval
// sum: accumulator for the Simpson's rule sum
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