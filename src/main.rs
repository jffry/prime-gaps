fn main() {
    // print a csv header
    println!("n, prime, elapsed_sec, gap2, gap4, gap6, gap8, ...");
    
    let mut gaps = vec![0u64];
    let report_interval = 1_000_000_000;
    let start_time = std::time::Instant::now();

    // start our search at 5, so that we never have to deal with the gap of 1 between 2 and 3
    // this lets us pack the gaps array more compactly - index 0 is gap 2, index 1 is gap 4, index 2 is gap 6, etc
    let primes = primal::Primes::all().enumerate();
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
            print!("{}, {}, {}", nth, prime, start_time.elapsed().as_secs_f64());
            for g in &gaps {
                print!(", {}", g);
            }
            println!("");
        }
        
        previous_prime = prime;
    });

}
