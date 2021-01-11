use std::char;

// Probably I could do this recursively as well, but I don't want to...
#[allow(dead_code)]
pub fn lexicographic_permutations(num_of_digits: usize, nth: usize) -> String {
    // Initialize chars to permute.
    // If num_of_digits is 3, then this initializes:
    // vec![Some(0), Some(1), Some(2)].
    let mut digits: Vec<Option<char>> = (0..num_of_digits)
        .map(|n| char::from_digit(n as u32, 10))
        .collect();

    let mut count = num_of_digits;
    let mut ans: Vec<char> = Vec::with_capacity(num_of_digits);

    // rem: remainder --- initial state being nth
    let mut rem = nth;
    while count != 1 {
        // divisor is a factorial of count as count keeps decrementing by 1
        // every iteration until it reaches 1.
        let divisor = (1..count).fold(1, |acc, x| acc * x);
        // If nth is the very last permutation, then return the reverse of digits.
        if divisor * num_of_digits == nth {
            return digits
                .into_iter()
                .rev()
                .map(|c| c.unwrap())
                .collect::<String>();
        }
        match (rem, divisor) {
            // If remainder(initial numerator) equals divisor,
            // push the very first digit to `ans` and concatinate the reverse of the rest to it.
            (r, d) if r == d => {
                ans.push(digits.remove(0).unwrap());
                let rev: Vec<char> = digits.into_iter().rev().map(|i| i.unwrap()).collect();
                ans = vec![ans, rev].into_iter().flatten().collect();
                let answer = ans.iter().collect::<String>();
                return answer;
            }
            _ => {
                let quo = rem.div_euclid(divisor);
                rem = rem.rem_euclid(divisor);
                match rem {
                    0 => {
                        // If remainder is 0, then get (quo - 1)th digit and set remainder to the latest quotient.
                        ans.push(digits.remove(quo - 1).unwrap());
                        rem = quo;
                    }
                    _ => ans.push(digits.remove(quo).unwrap()),
                }
            }
        }
        count -= 1;
    }
    // Lastly pop the last digit and push it to `ans`.
    ans.push(digits.pop().unwrap().unwrap());
    ans.iter().collect::<String>()
}
