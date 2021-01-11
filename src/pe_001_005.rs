#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn largest_prime_factor(mut num: u64) -> u64 {
    (2..)
        .scan(2, |_, x| {
            while num % x == 0 {
                println!("{}", x);
                num /= x;
            }
            if num != 1 {
                Some(x)
            } else {
                None
            }
        })
        .last()
        .unwrap()
        + 1
}

#[allow(dead_code)]
pub fn largest_palindrome_product(digits: usize, top_n_largest: usize) -> u64 {
    let from = 10usize.pow(digits as u32 - 1);
    let to = 10usize.pow(digits as u32);
    let mut count = 0;
    let mut largest = 0;
    for i in (from..to).rev() {
        for j in (i..to).rev() {
            let tmp = i * j;
            let num = tmp.to_string();
            let rev_num: String = num.chars().rev().collect();
            if num == rev_num {
                if tmp > largest {
                    largest = tmp;
                }
                println!("{} * {} ===> {}", i, j, num);
                count += 1;
                break;
            }
        }
        if count == top_n_largest {
            break;
        }
    }
    largest as u64
}
