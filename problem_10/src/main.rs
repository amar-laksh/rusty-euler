fn prime_sieve() -> [bool; 2_000_000] {
    const SIZE: usize = 2_000_000;
    let mut slots: [bool; SIZE] = [true; SIZE];
    slots[0] = false;
    slots[1] = false;

    // We calculate the primes using a simple stride and marking off multiples
    for stride in 2..(SIZE / 2) {
        let mut pos = stride;
        while pos < (SIZE - stride) {
            pos += stride;
            slots[pos] = false;
        }
    }
    return slots;
}

fn sum_prime() -> usize {
    let mut sum: usize = 0;
    prime_sieve()
        .iter()
        .enumerate()
        .into_iter()
        .for_each(|(idx, pr)| {
            if *pr {
                sum += idx;
            }
        });
    return sum;
}

fn main() {
    {
        println!("{}", sum_prime());
    }
}
