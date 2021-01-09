pub fn multiples_of_3_and_5(bound: u32) -> u32 {
    // let v: Vec<u32> = (3..bound)
    //     .into_iter()
    //     .filter(|n| n % 3 == 0 || n % 5 == 0)
    //     .collect();
    // println!("{:?}", v);
    // v.iter().sum::<u32>()
    (3..bound)
        .into_iter()
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

pub fn even_fib_numbers(bound: u64) -> u64 {
    (0..)
        .scan((0, 1), |s, _| {
            *s = (s.1, s.0 + s.1);
            if s.1 < bound {
                Some(s.1)
            } else {
                None
            }
        })
        .filter(|n| n % 2 == 0)
        .sum::<u64>()
}

pub fn largest_prime_factor(num: u32) -> u32 {
    0
}
