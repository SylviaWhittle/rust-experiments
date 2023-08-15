pub fn find_primes(max_number: i32) -> i32 {
    let mut i = 3;
    let mut latest_prime = 2;
    while i < max_number {
        let mut found_factor = false;
        for val in (3..(i as f64).sqrt().floor() as i32 + 1).step_by(2) {
            if i % val == 0 {
                found_factor = true;
                break;
            }
        }

        if !found_factor {
            // Copy the value
            latest_prime = i;
        }

        i += 2;
    }

    latest_prime
}
