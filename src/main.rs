use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive, Zero};
use primal::Primes;

/*
Computes a simple linear regression (slope, intercept, R²) as explained in
https://en.wikipedia.org/wiki/Simple_linear_regression#Fitting_the_regression_line
*/
fn stats(gaps: &Vec<u64>) -> (f64, f64, f64) {
    let points = gaps.iter().enumerate()
        .filter(|(_, count)| **count > 0)
        .map(|(index, count)| {
            (BigDecimal::from_usize((index + 1) << 1).unwrap(),
             BigDecimal::from_f64((*count as f64).log10()).unwrap())
        })
        .collect::<Vec<_>>();
    let n = &BigDecimal::from_usize(points.len()).unwrap();
    let mut x_sum = BigDecimal::zero();
    let mut y_sum = BigDecimal::zero();
    let mut xy_sum = BigDecimal::zero();
    let mut x2_sum = BigDecimal::zero();
    let mut y2_sum = BigDecimal::zero();
    for (x, y) in &points {
        x_sum += x;
        y_sum += y;
        x2_sum += x * x;
        xy_sum += x * y;
        y2_sum += y * y;
    }
    let x_mean = &(x_sum / n);
    let y_mean = &(y_sum / n);
    let x2_mean = &(x2_sum / n);
    let xy_mean = &(xy_sum / n);
    let y2_mean = &(y2_sum / n);
    let mut dx2_sum = BigDecimal::zero();
    let mut dxdy_sum = BigDecimal::zero();
    for (x, y) in &points {
        let dx = &(x - x_mean);
        dx2_sum += dx * dx;
        dxdy_sum += dx * (y - y_mean);
    }
    let slope = &(dxdy_sum / dx2_sum);
    let intercept = y_mean - slope * x_mean;
    let r2_numerator = (xy_mean - x_mean * y_mean) * (xy_mean - x_mean * y_mean);
    let r2_denominator = (x2_mean - (x_mean * x_mean)) * (y2_mean - (y_mean * y_mean));
    let r2 = r2_numerator / r2_denominator;
    (slope.to_f64().unwrap(), intercept.to_f64().unwrap(), r2.to_f64().unwrap())
}

fn generate_forever() {
    // print a csv header
    println!("n,prime_n,elapsed_sec,|,linreg_slope,linreg_intercept,linreg_R2,|,gap2,gap4,gap6,gap8,...");

    let mut gaps = vec![0u64];
    let report_interval = 1_000_000_000;
    let start_time = std::time::Instant::now();

    // start our search at 5, so that we never have to deal with the gap of 1 between 2 and 3
    // this lets us pack the gaps array more compactly - index 0 is gap 2, index 1 is gap 4, index 2 is gap 6, etc
    let primes = Primes::all().enumerate();
    let mut previous_prime = 3;
    let mut next_report = report_interval;

    primes.skip(2).for_each(|(index, prime)| {
        let nth = index + 1; //prime at index 0 is the first prime
        let gap = prime - previous_prime;
        let gap_index = (gap >> 1) - 1; // gaps[0] is the 2-gap tally, gaps[1] is 4, gaps[2] is 6, etc

        // grow the vector to be able to store the full list
        // this happens very infrequently
        if gap_index >= gaps.len() {
            gaps.resize(gap_index + 1, 0);
        }
        gaps[gap_index] += 1;

        // print out a report if we are at a significant index
        // equality checking like this is a lot faster than doing a % on every iteration
        if nth == next_report {
            next_report += report_interval;
            let elapsed = start_time.elapsed().as_secs_f64();
            let (linreg_slope, linreg_intercept, linreg_r2) = stats(&gaps);
            print!("{},{},{},|,{},{},{},|", nth, prime, elapsed, linreg_slope, linreg_intercept, linreg_r2);
            for g in &gaps {
                print!(",{}", g);
            }
            println!();
        }

        previous_prime = prime;
    });
}

fn main() {
    //TODO: if Primal implements a way to serialize its internal state, we should persist that
    //to disk so the computation can be paused and resumed - https://github.com/huonw/primal/issues/33
    generate_forever();
}
