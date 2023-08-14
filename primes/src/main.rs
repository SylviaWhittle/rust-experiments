use std::time::Instant;

use primes::find_primes;

fn main() {
    let max_number = 1000000;
    let now = Instant::now();
    let last_prime = { find_primes(max_number) };
    let elapsed = now.elapsed();
    // :.2? means display as a floating point number with 2dp
    // The ? means debug
    println!(
        "Found primes up to {max_number} in {:.2?}. Last prime: {last_prime}",
        elapsed
    );
}
