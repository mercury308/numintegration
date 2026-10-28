// Left Riemann Sum 
// f: function to integrate
// a: lower limit
// b: upper limit
// n: number of subintervals
// i: index of the subinterval (starts from 0 for left Riemann sum)
// h: width of each subinterval
// sum: accumulator for the Riemann sum
fn left_rs(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.0;
    for i in 0..n {
        sum += f(a + i as f64 * h);
    }
    sum * h
}

