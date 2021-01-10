use std::cmp::Ordering::*;

#[allow(dead_code)]
pub fn largest_product_in_a_series(n: u8) -> u64 {
    let filename = "inputs/input8.txt";
    let lines = readfile!(filename);
    // println!("{:?}", lines);
    // for line in lines.iter() {
    //     println!("{}", line);
    // }
    let oneline = lines.join("");
    let length = oneline.len();
    println!("{}", length);
    let mut largest = 0;
    match length.cmp(&(n as usize)) {
        Less => println!("The input size should not be smaller than n."),
        _ => {
            let vec = series(&oneline, n as usize, length);
            for v in &vec {
                if !v.chars().into_iter().any(|c| c == '0') {
                    let tmp = v
                        .chars()
                        .into_iter()
                        .fold(1, |acc, x| acc * (x as u64 - 48));
                    if tmp > largest {
                        println!("{:?} ===> {}", v, tmp);
                        largest = tmp;
                    }
                }
            }
        }
    }
    largest
}

fn series(digits: &str, len: usize, length: usize) -> Vec<String> {
    (0..length + 1 - len)
        .scan("".to_string(), |_, x| {
            let num = digits.get(x..x + len).unwrap();
            Some(num.to_string())
        })
        .collect::<Vec<String>>()
}

#[allow(dead_code)]
// Pythagorean triplets: a^2 + b^2 = c^2 where a < b < c.
// Find triplets that satisfies a + b + c = 1000
// and the product of them.
// To begin with, a^2 + b^2 - c^2 = 0
//            => (a + b)^2 - c^2 = 2ab
//            => (a + b + c)(a + b - c) = 2ab.
// And in this case, replacing (a + b + c) with 1000, we have a + b = ab/500 + c.
// Hence, we want to find triplets `a, b, c` that satisfy:
// 1. ab is divisible by 500,
// 2. a + b = (ab/500) + c,
// 3. a + b + c = 1000.
// Return 0 if no such triplets.
pub fn special_pythagorean_triplet(target: u64) -> u64 {
    let divisor = target / 2;
    for a in 1..1000 {
        for b in a + 1..1000 {
            let ab = a * b;
            if (ab) % divisor == 0 {
                let div = ab / divisor;
                for c in b + 1..1000 {
                    let addition = a + b;
                    if addition == div + c && a + b + c == target {
                        println!("a = {}\nb = {}\nc = {}git ", a, b, c);
                        return ab as u64 * c as u64;
                    }
                }
            }
        }
    }
    0
}
